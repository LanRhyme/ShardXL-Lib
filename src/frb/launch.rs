// Copyright (c) 2025 ShardXL
// Licensed under the MIT License

//! Launch FFI functions for flutter_rust_bridge

use flutter_rust_bridge::frb;
use crate::frb::models::{LaunchConfig, LaunchResult, InstallResult};
use std::process::Command;

/// Check if Java is available and get version string
#[frb]
pub fn check_java(java_path: Option<String>) -> String {
    let java_executable = if let Some(path) = java_path {
        path
    } else {
        // Try to find Java
        if let Ok(java_home) = std::env::var("JAVA_HOME") {
            format!("{}/bin/java.exe", java_home)
        } else {
            "java".to_string()
        }
    };
    
    let output = Command::new(&java_executable)
        .args(["-version"])
        .output();
    
    match output {
        Ok(out) => {
            let stderr = String::from_utf8_lossy(&out.stderr);
            // Parse version from output like "java version \"21.0.1\""
            if let Some(start) = stderr.find("\"") {
                let rest = &stderr[start + 1..];
                if let Some(end) = rest.find("\"") {
                    return rest[..end].to_string();
                }
            }
            "Unknown".to_string()
        }
        Err(_) => "Java not found".to_string(),
    }
}

/// Download version JSON synchronously
#[frb]
pub fn download_version_json(version_id: String, game_directory: String) -> Result<String, String> {
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| e.to_string())?;
    
    // First get manifest
    let manifest_response = client
        .get("https://piston-meta.mojang.com/mc/game/version_manifest_v2.json")
        .send()
        .map_err(|e| e.to_string())?;
    
    let manifest: serde_json::Value = manifest_response.json().map_err(|e| e.to_string())?;
    
    // Find version URL
    let version_url = manifest["versions"]
        .as_array()
        .ok_or("Invalid manifest format")?
        .iter()
        .find(|v| v["id"].as_str() == Some(&version_id))
        .ok_or(format!("Version {} not found in manifest", version_id))?
        .get("url")
        .ok_or("Missing version URL")?
        .as_str()
        .ok_or("Invalid version URL")?
        .to_string();
    
    // Download version JSON
    let version_json: serde_json::Value = client
        .get(&version_url)
        .send()
        .map_err(|e| e.to_string())?
        .json()
        .map_err(|e| e.to_string())?;
    
    // Save to disk
    let versions_dir = std::path::Path::new(&game_directory).join("versions").join(&version_id);
    std::fs::create_dir_all(&versions_dir).map_err(|e| e.to_string())?;
    
    let json_path = versions_dir.join(format!("{}.json", version_id));
    let json_content = serde_json::to_string_pretty(&version_json).map_err(|e| e.to_string())?;
    std::fs::write(&json_path, json_content).map_err(|e| e.to_string())?;
    
    Ok(json_path.to_string_lossy().to_string())
}

/// Download client.jar synchronously
#[frb]
pub fn download_client_jar(version_id: String, game_directory: String) -> Result<String, String> {
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(120))
        .build()
        .map_err(|e| e.to_string())?;
    
    // Read version JSON
    let json_path = std::path::Path::new(&game_directory)
        .join("versions")
        .join(&version_id)
        .join(format!("{}.json", version_id));
    
    let version_json: serde_json::Value = if json_path.exists() {
        let content = std::fs::read_to_string(&json_path).map_err(|e| e.to_string())?;
        serde_json::from_str(&content).map_err(|e| e.to_string())?
    } else {
        // Download version JSON first
        let _ = download_version_json(version_id.clone(), game_directory.clone())?;
        let content = std::fs::read_to_string(&json_path).map_err(|e| e.to_string())?;
        serde_json::from_str(&content).map_err(|e| e.to_string())?
    };
    
    // Get client download URL
    let downloads = version_json.get("downloads")
        .ok_or("Missing downloads in version JSON")?;
    let client_download = downloads.get("client")
        .ok_or("Missing client download info")?;
    let jar_url = client_download.get("url")
        .ok_or("Missing client JAR URL")?
        .as_str()
        .ok_or("Invalid client JAR URL")?;
    
    // Download client.jar
    let versions_dir = std::path::Path::new(&game_directory).join("versions").join(&version_id);
    let jar_path = versions_dir.join(format!("{}.jar", version_id));
    
    let response = client.get(jar_url).send().map_err(|e| e.to_string())?;
    let bytes = response.bytes().map_err(|e| e.to_string())?;
    std::fs::write(&jar_path, bytes).map_err(|e| e.to_string())?;
    
    Ok(jar_path.to_string_lossy().to_string())
}

/// Install a Minecraft version (download required files)
#[frb]
pub fn install_version(version_id: String, game_directory: String) -> InstallResult {
    use std::path::Path;
    
    let versions_dir = Path::new(&game_directory).join("versions").join(&version_id);
    
    // Create version directory
    if let Err(e) = std::fs::create_dir_all(&versions_dir) {
        return InstallResult {
            success: false,
            error: Some(format!("Failed to create version directory: {}", e)),
        };
    }
    
    // Download version JSON
    if let Err(e) = download_version_json(version_id.clone(), game_directory.clone()) {
        return InstallResult {
            success: false,
            error: Some(e),
        };
    }
    
    // Download client.jar
    if let Err(e) = download_client_jar(version_id.clone(), game_directory.clone()) {
        return InstallResult {
            success: false,
            error: Some(e),
        };
    }
    
    InstallResult {
        success: true,
        error: None,
    }
}

/// Get installed versions list
#[frb]
pub fn get_installed_versions(game_directory: String) -> Vec<String> {
    use std::fs;
    
    let versions_dir = std::path::Path::new(&game_directory).join("versions");
    
    if !versions_dir.exists() {
        return Vec::new();
    }
    
    let mut versions = Vec::new();
    
    if let Ok(entries) = fs::read_dir(&versions_dir) {
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                if let Some(name) = entry.path().file_name() {
                    let name_str = name.to_string_lossy().to_string();
                    // Check if version has required files
                    let jar_path = entry.path().join(format!("{}.jar", name_str));
                    let json_path = entry.path().join(format!("{}.json", name_str));
                    if jar_path.exists() && json_path.exists() {
                        versions.push(name_str);
                    }
                }
            }
        }
    }
    
    versions
}

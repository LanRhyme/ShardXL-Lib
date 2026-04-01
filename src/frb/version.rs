// Copyright (c) 2025 ShardXL
// Licensed under the MIT License

//! Version FFI functions for flutter_rust_bridge

use flutter_rust_bridge::frb;
use crate::frb::models::{MinecraftVersion, VersionListResult};

/// Fetch version manifest from Mojang
#[frb]
pub fn fetch_version_manifest() -> Result<Vec<MinecraftVersion>, String> {
    // Synchronous HTTP request using blocking reqwest
    let client = reqwest::blocking::Client::new();
    
    let response = client
        .get("https://piston-meta.mojang.com/mc/game/version_manifest_v2.json")
        .send()
        .map_err(|e| e.to_string())?;
    
    let manifest: serde_json::Value = response.json().map_err(|e| e.to_string())?;
    
    let versions_json = manifest
        .get("versions")
        .ok_or("Missing 'versions' in manifest")?;
    
    let versions = versions_json
        .as_array()
        .ok_or("'versions' is not an array")?
        .iter()
        .filter_map(|v| {
            let id = v.get("id")?.as_str()?.to_string();
            let version_type = v.get("type")?.as_str()?.to_string();
            let release_time = v.get("releaseTime")?.as_str()?.to_string();
            
            Some(MinecraftVersion {
                id,
                version_type,
                release_time,
            })
        })
        .collect();
    
    Ok(versions)
}

/// Get all available versions (calls fetch_version_manifest)
#[frb]
pub fn get_available_versions() -> VersionListResult {
    match fetch_version_manifest() {
        Ok(versions) => VersionListResult {
            versions,
            error: None,
        },
        Err(e) => VersionListResult {
            versions: Vec::new(),
            error: Some(e),
        },
    }
}

/// Get release versions only
#[frb]
pub fn get_release_versions() -> VersionListResult {
    let result = get_available_versions();
    
    if result.error.is_some() {
        return result;
    }
    
    let release_versions: Vec<MinecraftVersion> = result
        .versions
        .into_iter()
        .filter(|v| v.version_type == "release")
        .collect();
    
    VersionListResult {
        versions: release_versions,
        error: None,
    }
}

/// Get versions by type (release, snapshot, old_beta, old_alpha)
#[frb]
pub fn get_versions_by_type(version_type: String) -> VersionListResult {
    let result = get_available_versions();
    
    if result.error.is_some() {
        return result;
    }
    
    let filtered_versions: Vec<MinecraftVersion> = result
        .versions
        .into_iter()
        .filter(|v| v.version_type == version_type)
        .collect();
    
    VersionListResult {
        versions: filtered_versions,
        error: None,
    }
}

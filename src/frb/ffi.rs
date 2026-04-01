// Copyright (c) 2025 ShardXL
// Licensed under the MIT License

//! C-compatible FFI implementation for flutter_rust_bridge alternative
//! Uses #[no_mangle] and extern "C" for C ABI compatibility

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

use crate::auth::generate_offline_uuid;
use super::models::{UserProfile, MinecraftVersion, AuthResult};

#[cfg(target_os = "windows")]
use std::env;

// ============================================================================
// C-Compatible Type Definitions
// ============================================================================

#[repr(C)]
pub struct ShardXL_UserProfile {
    pub id: u64,
    pub username: *mut c_char,
    pub uuid: *mut c_char,
    pub access_token: *mut c_char,
    pub email: *mut c_char,
    pub email_verified: bool,
    pub banned: bool,
}

#[repr(C)]
pub struct ShardXL_MinecraftVersion {
    pub id: *mut c_char,
    pub version_type: *mut c_char,
    pub release_time: *mut c_char,
}

#[repr(C)]
pub struct ShardXL_AuthResult {
    pub success: bool,
    pub profile: *mut ShardXL_UserProfile,
    pub error: *mut c_char,
}

#[repr(C)]
pub struct ShardXL_VersionListResult {
    pub versions: *mut ShardXL_MinecraftVersion,
    pub versions_count: usize,
    pub error: *mut c_char,
}

#[repr(C)]
pub struct ShardXL_LaunchConfig {
    pub version: *mut c_char,
    pub game_directory: *mut c_char,
    pub java_path: *mut c_char,
    pub memory_mb: u32,
    pub username: *mut c_char,
    pub uuid: *mut c_char,
    pub access_token: *mut c_char,
    pub window_width: u32,
    pub window_height: u32,
    pub fullscreen: bool,
}

#[repr(C)]
pub struct ShardXL_LaunchResult {
    pub success: bool,
    pub pid: u32,
    pub error: *mut c_char,
}

#[repr(C)]
pub struct ShardXL_InstallResult {
    pub success: bool,
    pub error: *mut c_char,
}

#[repr(C)]
pub struct ShardXL_JavaInfo {
    pub path: *mut c_char,
    pub version: *mut c_char,
    pub major_version: u8,
}

// ============================================================================
// Memory Management Functions
// ============================================================================

#[no_mangle]
pub extern "C" fn shardxl_free_string(str: *mut c_char) {
    if !str.is_null() {
        unsafe {
            String::from_raw_parts(str as *mut u8, 0, 0);
        }
    }
}

#[no_mangle]
pub extern "C" fn shardxl_free_user_profile(profile: *mut ShardXL_UserProfile) {
    if !profile.is_null() {
        unsafe {
            let p = &mut *profile;
            if !p.username.is_null() {
                String::from_raw_parts(p.username as *mut u8, 0, 0);
            }
            if !p.uuid.is_null() {
                String::from_raw_parts(p.uuid as *mut u8, 0, 0);
            }
            if !p.access_token.is_null() {
                String::from_raw_parts(p.access_token as *mut u8, 0, 0);
            }
            if !p.email.is_null() {
                String::from_raw_parts(p.email as *mut u8, 0, 0);
            }
            ptr::drop_in_place(p);
        }
    }
}

#[no_mangle]
pub extern "C" fn shardxl_free_auth_result(result: *mut ShardXL_AuthResult) {
    if !result.is_null() {
        unsafe {
            let r = &mut *result;
            if !r.error.is_null() {
                String::from_raw_parts(r.error as *mut u8, 0, 0);
            }
            if !r.profile.is_null() {
                shardxl_free_user_profile(r.profile);
            }
            ptr::drop_in_place(r);
        }
    }
}

#[no_mangle]
pub extern "C" fn shardxl_free_version_list_result(result: *mut ShardXL_VersionListResult) {
    if !result.is_null() {
        unsafe {
            let r = &mut *result;
            if !r.error.is_null() {
                String::from_raw_parts(r.error as *mut u8, 0, 0);
            }
            if !r.versions.is_null() {
                for i in 0..r.versions_count {
                    let v = &mut *r.versions.add(i);
                    if !v.id.is_null() {
                        String::from_raw_parts(v.id as *mut u8, 0, 0);
                    }
                    if !v.version_type.is_null() {
                        String::from_raw_parts(v.version_type as *mut u8, 0, 0);
                    }
                    if !v.release_time.is_null() {
                        String::from_raw_parts(v.release_time as *mut u8, 0, 0);
                    }
                }
                std::alloc::dealloc(
                    r.versions as *mut u8,
                    std::alloc::Layout::array::<ShardXL_MinecraftVersion>(r.versions_count).unwrap(),
                );
            }
            ptr::drop_in_place(r);
        }
    }
}

#[no_mangle]
pub extern "C" fn shardxl_free_launch_result(result: *mut ShardXL_LaunchResult) {
    if !result.is_null() {
        unsafe {
            let r = &mut *result;
            if !r.error.is_null() {
                String::from_raw_parts(r.error as *mut u8, 0, 0);
            }
            ptr::drop_in_place(r);
        }
    }
}

#[no_mangle]
pub extern "C" fn shardxl_free_install_result(result: *mut ShardXL_InstallResult) {
    if !result.is_null() {
        unsafe {
            let r = &mut *result;
            if !r.error.is_null() {
                String::from_raw_parts(r.error as *mut u8, 0, 0);
            }
            ptr::drop_in_place(r);
        }
    }
}

#[no_mangle]
pub extern "C" fn shardxl_free_java_info(info: *mut ShardXL_JavaInfo) {
    if !info.is_null() {
        unsafe {
            let i = &mut *info;
            if !i.path.is_null() {
                String::from_raw_parts(i.path as *mut u8, 0, 0);
            }
            if !i.version.is_null() {
                String::from_raw_parts(i.version as *mut u8, 0, 0);
            }
            ptr::drop_in_place(i);
        }
    }
}

#[no_mangle]
pub extern "C" fn shardxl_free_string_array(array: *mut *mut c_char, count: usize) {
    if !array.is_null() {
        unsafe {
            for i in 0..count {
                let s = *array.add(i);
                if !s.is_null() {
                    String::from_raw_parts(s as *mut u8, 0, 0);
                }
            }
            std::alloc::dealloc(
                array as *mut u8,
                std::alloc::Layout::array::<*mut c_char>(count).unwrap(),
            );
        }
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

fn to_c_string(s: &str) -> *mut c_char {
    CString::new(s).unwrap().into_raw()
}

fn from_c_str(ptr: *const c_char) -> String {
    if ptr.is_null() {
        String::new()
    } else {
        unsafe { CStr::from_ptr(ptr).to_string_lossy().into_owned() }
    }
}

// ============================================================================
// Authentication Functions
// ============================================================================

#[no_mangle]
pub extern "C" fn shardxl_authenticate_offline(username: *const c_char) -> *mut ShardXL_AuthResult {
    let username_str = from_c_str(username);
    
    let result = if username_str.is_empty() {
        AuthResult {
            success: false,
            profile: None,
            error: Some("Username cannot be empty".to_string()),
        }
    } else if username_str.len() < 3 || username_str.len() > 16 {
        AuthResult {
            success: false,
            profile: None,
            error: Some("Username must be between 3 and 16 characters".to_string()),
        }
    } else {
        let uuid = generate_offline_uuid(&username_str);
        AuthResult {
            success: true,
            profile: Some(UserProfile {
                id: None,
                username: username_str.clone(),
                uuid: uuid.clone(),
                access_token: None,
                email: None,
                email_verified: false,
                banned: false,
            }),
            error: None,
        }
    };

    Box::into_raw(Box::new(ShardXL_AuthResult {
        success: result.success,
        profile: result.profile.map(|p| Box::into_raw(Box::new(ShardXL_UserProfile {
            id: p.id.unwrap_or(0),
            username: to_c_string(&p.username),
            uuid: to_c_string(&p.uuid),
            access_token: p.access_token.as_ref().map(|s| to_c_string(s)).unwrap_or(std::ptr::null_mut()),
            email: p.email.as_ref().map(|s| to_c_string(s)).unwrap_or(std::ptr::null_mut()),
            email_verified: p.email_verified,
            banned: p.banned,
        }))).unwrap_or(std::ptr::null_mut()),
        error: result.error.as_ref().map(|s| to_c_string(s)).unwrap_or(std::ptr::null_mut()),
    }))
}

#[no_mangle]
pub extern "C" fn shardxl_get_default_game_directory() -> *mut c_char {
    #[cfg(target_os = "windows")]
    {
        if let Ok(appdata) = env::var("APPDATA") {
            return to_c_string(&format!("{}/.minecraft", appdata));
        }
    }
    #[cfg(target_os = "macos")]
    {
        if let Ok(home) = env::var("HOME") {
            return to_c_string(&format!("{}/Library/Application Support/minecraft", home));
        }
    }
    #[cfg(target_os = "linux")]
    {
        if let Ok(home) = env::var("HOME") {
            return to_c_string(&format!("{}/.minecraft", home));
        }
    }
    to_c_string("")
}

#[no_mangle]
pub extern "C" fn shardxl_is_version_installed(version_id: *const c_char, game_directory: *const c_char) -> bool {
    use std::path::Path;
    
    let version_id = from_c_str(version_id);
    let game_directory = from_c_str(game_directory);
    
    let versions_dir = Path::new(&game_directory).join("versions").join(&version_id);
    let jar_path = versions_dir.join(format!("{}.jar", version_id));
    let json_path = versions_dir.join(format!("{}.json", version_id));
    
    jar_path.exists() && json_path.exists()
}

// ============================================================================
// Version Functions
// ============================================================================

fn fetch_version_manifest() -> Result<Vec<MinecraftVersion>, String> {
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

fn create_version_list_result(versions: Vec<MinecraftVersion>, error: Option<String>) -> *mut ShardXL_VersionListResult {
    let count = versions.len();
    let mut versions_box: Vec<ShardXL_MinecraftVersion> = versions
        .into_iter()
        .map(|v| ShardXL_MinecraftVersion {
            id: to_c_string(&v.id),
            version_type: to_c_string(&v.version_type),
            release_time: to_c_string(&v.release_time),
        })
        .collect();
    
    let versions_ptr = versions_box.as_mut_ptr();
    std::mem::forget(versions_box);
    
    Box::into_raw(Box::new(ShardXL_VersionListResult {
        versions: versions_ptr,
        versions_count: count,
        error: error.as_ref().map(|s| to_c_string(s)).unwrap_or(std::ptr::null_mut()),
    }))
}

#[no_mangle]
pub extern "C" fn shardxl_get_available_versions() -> *mut ShardXL_VersionListResult {
    match fetch_version_manifest() {
        Ok(versions) => create_version_list_result(versions, None),
        Err(e) => create_version_list_result(Vec::new(), Some(e)),
    }
}

#[no_mangle]
pub extern "C" fn shardxl_get_release_versions() -> *mut ShardXL_VersionListResult {
    match fetch_version_manifest() {
        Ok(versions) => {
            let release_versions: Vec<MinecraftVersion> = versions
                .into_iter()
                .filter(|v| v.version_type == "release")
                .collect();
            create_version_list_result(release_versions, None)
        }
        Err(e) => create_version_list_result(Vec::new(), Some(e)),
    }
}

#[no_mangle]
pub extern "C" fn shardxl_get_versions_by_type(version_type: *const c_char) -> *mut ShardXL_VersionListResult {
    let version_type = from_c_str(version_type);
    
    match fetch_version_manifest() {
        Ok(versions) => {
            let filtered: Vec<MinecraftVersion> = versions
                .into_iter()
                .filter(|v| v.version_type == version_type)
                .collect();
            create_version_list_result(filtered, None)
        }
        Err(e) => create_version_list_result(Vec::new(), Some(e)),
    }
}

// ============================================================================
// Launch Functions
// ============================================================================

#[no_mangle]
pub extern "C" fn shardxl_check_java(java_path: *const c_char) -> *mut c_char {
    let java_path_str = from_c_str(java_path);
    
    let java_executable = if java_path_str.is_empty() {
        if let Ok(java_home) = env::var("JAVA_HOME") {
            if cfg!(target_os = "windows") {
                format!("{}/bin/java.exe", java_home)
            } else {
                format!("{}/bin/java", java_home)
            }
        } else {
            "java".to_string()
        }
    } else {
        java_path_str
    };
    
    let output = std::process::Command::new(&java_executable)
        .args(["-version"])
        .output();
    
    match output {
        Ok(out) => {
            let stderr = String::from_utf8_lossy(&out.stderr);
            if let Some(start) = stderr.find('"') {
                let rest = &stderr[start + 1..];
                if let Some(end) = rest.find('"') {
                    return to_c_string(&rest[..end]);
                }
            }
            to_c_string("Unknown")
        }
        Err(_) => to_c_string("Java not found"),
    }
}

#[no_mangle]
pub extern "C" fn shardxl_download_version_json(version_id: *const c_char, game_directory: *const c_char) -> *mut c_char {
    let version_id = from_c_str(version_id);
    let game_directory = from_c_str(game_directory);
    
    let client = reqwest::blocking::Client::new();
    
    let manifest_response = match client
        .get("https://piston-meta.mojang.com/mc/game/version_manifest_v2.json")
        .send()
    {
        Ok(resp) => resp,
        Err(e) => return to_c_string(&format!("Failed to fetch manifest: {}", e)),
    };
    
    let manifest: serde_json::Value = match manifest_response.json() {
        Ok(m) => m,
        Err(e) => return to_c_string(&format!("Failed to parse manifest: {}", e)),
    };
    
    let version_url = match manifest["versions"]
        .as_array()
        .and_then(|arr| arr.iter().find(|v| v["id"].as_str() == Some(&version_id)))
        .and_then(|v| v.get("url"))
        .and_then(|u| u.as_str())
    {
        Some(url) => url.to_string(),
        None => return to_c_string(&format!("Version {} not found in manifest", version_id)),
    };
    
    let version_json: serde_json::Value = match client.get(&version_url).send() {
        Ok(resp) => match resp.json() {
            Ok(json) => json,
            Err(e) => return to_c_string(&format!("Failed to parse version JSON: {}", e)),
        },
        Err(e) => return to_c_string(&format!("Failed to download version JSON: {}", e)),
    };
    
    let versions_dir = std::path::Path::new(&game_directory).join("versions").join(&version_id);
    if let Err(e) = std::fs::create_dir_all(&versions_dir) {
        return to_c_string(&format!("Failed to create version directory: {}", e));
    }
    
    let json_path = versions_dir.join(format!("{}.json", version_id));
    let json_content = match serde_json::to_string_pretty(&version_json) {
        Ok(c) => c,
        Err(e) => return to_c_string(&format!("Failed to serialize JSON: {}", e)),
    };
    
    if let Err(e) = std::fs::write(&json_path, json_content) {
        return to_c_string(&format!("Failed to write JSON file: {}", e));
    }
    
    to_c_string(&json_path.to_string_lossy())
}

#[no_mangle]
pub extern "C" fn shardxl_download_client_jar(version_id: *const c_char, game_directory: *const c_char) -> *mut c_char {
    let version_id = from_c_str(version_id);
    let game_directory = from_c_str(game_directory);
    
    let client = reqwest::blocking::Client::new();
    
    let json_path = std::path::Path::new(&game_directory)
        .join("versions")
        .join(&version_id)
        .join(format!("{}.json", version_id));
    
    let version_json: serde_json::Value = if json_path.exists() {
        match std::fs::read_to_string(&json_path) {
            Ok(content) => match serde_json::from_str(&content) {
                Ok(json) => json,
                Err(e) => return to_c_string(&format!("Failed to parse existing JSON: {}", e)),
            },
            Err(e) => return to_c_string(&format!("Failed to read JSON file: {}", e)),
        }
    } else {
        let manifest_response = match client
            .get("https://piston-meta.mojang.com/mc/game/version_manifest_v2.json")
            .send()
        {
            Ok(resp) => resp,
            Err(e) => return to_c_string(&format!("Failed to fetch manifest: {}", e)),
        };
        
        let manifest: serde_json::Value = match manifest_response.json() {
            Ok(m) => m,
            Err(e) => return to_c_string(&format!("Failed to parse manifest: {}", e)),
        };
        
        let version_url = match manifest["versions"]
            .as_array()
            .and_then(|arr| arr.iter().find(|v| v["id"].as_str() == Some(&version_id)))
            .and_then(|v| v.get("url"))
            .and_then(|u| u.as_str())
        {
            Some(url) => url.to_string(),
            None => return to_c_string(&format!("Version {} not found in manifest", version_id)),
        };
        
        match client.get(&version_url).send() {
            Ok(resp) => match resp.json() {
                Ok(json) => json,
                Err(e) => return to_c_string(&format!("Failed to parse version JSON: {}", e)),
            },
            Err(e) => return to_c_string(&format!("Failed to download version JSON: {}", e)),
        }
    };
    
    let jar_url = match version_json
        .get("downloads")
        .and_then(|d| d.get("client"))
        .and_then(|c| c.get("url"))
        .and_then(|u| u.as_str())
    {
        Some(url) => url,
        None => return to_c_string("Missing client JAR URL in version JSON"),
    };
    
    let versions_dir = std::path::Path::new(&game_directory).join("versions").join(&version_id);
    let jar_path = versions_dir.join(format!("{}.jar", version_id));
    
    let response = match client.get(jar_url).send() {
        Ok(resp) => resp,
        Err(e) => return to_c_string(&format!("Failed to download JAR: {}", e)),
    };
    
    let bytes = match response.bytes() {
        Ok(b) => b,
        Err(e) => return to_c_string(&format!("Failed to read JAR bytes: {}", e)),
    };
    
    if let Err(e) = std::fs::write(&jar_path, bytes) {
        return to_c_string(&format!("Failed to write JAR file: {}", e));
    }
    
    to_c_string(&jar_path.to_string_lossy())
}

#[no_mangle]
pub extern "C" fn shardxl_install_version(version_id: *const c_char, game_directory: *const c_char) -> *mut ShardXL_InstallResult {
    let version_id = from_c_str(version_id);
    let game_directory = from_c_str(game_directory);
    
    let versions_dir = std::path::Path::new(&game_directory).join("versions").join(&version_id);
    
    if let Err(e) = std::fs::create_dir_all(&versions_dir) {
        return Box::into_raw(Box::new(ShardXL_InstallResult {
            success: false,
            error: to_c_string(&format!("Failed to create version directory: {}", e)),
        }));
    }
    
    let json_result = shardxl_download_version_json(
        CString::new(version_id.clone()).unwrap().as_ptr(),
        CString::new(game_directory.clone()).unwrap().as_ptr(),
    );
    
    if from_c_str(json_result).starts_with("Failed") {
        let error = from_c_str(json_result);
        shardxl_free_string(json_result);
        return Box::into_raw(Box::new(ShardXL_InstallResult {
            success: false,
            error: to_c_string(&error),
        }));
    }
    shardxl_free_string(json_result);
    
    let jar_result = shardxl_download_client_jar(
        CString::new(version_id.clone()).unwrap().as_ptr(),
        CString::new(game_directory.clone()).unwrap().as_ptr(),
    );
    
    if from_c_str(jar_result).starts_with("Failed") {
        let error = from_c_str(jar_result);
        shardxl_free_string(jar_result);
        return Box::into_raw(Box::new(ShardXL_InstallResult {
            success: false,
            error: to_c_string(&error),
        }));
    }
    shardxl_free_string(jar_result);
    
    Box::into_raw(Box::new(ShardXL_InstallResult {
        success: true,
        error: std::ptr::null_mut(),
    }))
}

#[no_mangle]
pub extern "C" fn shardxl_get_installed_versions(game_directory: *const c_char, count: *mut usize) -> *mut *mut c_char {
    let game_directory = from_c_str(game_directory);
    
    let versions_dir = std::path::Path::new(&game_directory).join("versions");
    
    if !versions_dir.exists() {
        unsafe { *count = 0; }
        return ptr::null_mut();
    }
    
    let mut versions: Vec<String> = Vec::new();
    
    if let Ok(entries) = std::fs::read_dir(&versions_dir) {
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                if let Some(name) = entry.path().file_name() {
                    let name_str = name.to_string_lossy().to_string();
                    let jar_path = entry.path().join(format!("{}.jar", name_str));
                    let json_path = entry.path().join(format!("{}.json", name_str));
                    if jar_path.exists() && json_path.exists() {
                        versions.push(name_str);
                    }
                }
            }
        }
    }
    
    let count_val = versions.len();
    
    let mut result: Vec<*mut c_char> = versions
        .into_iter()
        .map(|s| to_c_string(&s))
        .collect();
    
    let ptr = result.as_mut_ptr();
    std::mem::forget(result);
    
    unsafe { *count = count_val; }
    ptr
}

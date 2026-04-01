// Copyright (c) 2025 ShardXL
// Licensed under the MIT License

//! Authentication FFI functions for flutter_rust_bridge

use flutter_rust_bridge::frb;
use crate::frb::models::{AuthResult, UserProfile};
use crate::auth::generate_offline_uuid;

/// Authenticate offline (no network)
/// Returns AuthResult with success status
#[frb]
pub fn authenticate_offline(username: String) -> AuthResult {
    if username.is_empty() {
        return AuthResult {
            success: false,
            profile: None,
            error: Some("Username cannot be empty".to_string()),
        };
    }
    
    if username.len() < 3 || username.len() > 16 {
        return AuthResult {
            success: false,
            profile: None,
            error: Some("Username must be between 3 and 16 characters".to_string()),
        };
    }
    
    let uuid = generate_offline_uuid(&username);
    
    AuthResult {
        success: true,
        profile: Some(UserProfile {
            id: None,
            username: username.clone(),
            uuid: uuid.clone(),
            access_token: None,
            email: None,
            email_verified: false,
            banned: false,
        }),
        error: None,
    }
}

/// Get default game directory based on platform
#[frb]
pub fn get_default_game_directory() -> String {
    #[cfg(target_os = "windows")]
    {
        if let Ok(appdata) = std::env::var("APPDATA") {
            return format!("{}/.minecraft", appdata);
        }
    }
    #[cfg(target_os = "macos")]
    {
        if let Ok(home) = std::env::var("HOME") {
            return format!("{}/Library/Application Support/minecraft", home);
        }
    }
    #[cfg(target_os = "linux")]
    {
        if let Ok(home) = std::env::var("HOME") {
            return format!("{}/.minecraft", home);
        }
    }
    
    String::new()
}

/// Check if a version is installed
#[frb]
pub fn is_version_installed(version_id: String, game_directory: String) -> bool {
    use std::path::Path;
    
    let versions_dir = Path::new(&game_directory).join("versions").join(&version_id);
    let jar_path = versions_dir.join(format!("{}.jar", version_id));
    let json_path = versions_dir.join(format!("{}.json", version_id));
    
    jar_path.exists() && json_path.exists()
}

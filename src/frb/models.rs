// Copyright (c) 2025 ShardXL
// Licensed under the MIT License

//! Data models for Flutter Rust Bridge

use serde::{Deserialize, Serialize};

/// User profile returned after successful authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: Option<u64>,
    pub username: String,
    pub uuid: String,
    pub access_token: Option<String>,
    pub email: Option<String>,
    pub email_verified: bool,
    pub banned: bool,
}

/// Minecraft version information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinecraftVersion {
    pub id: String,
    pub version_type: String,
    pub release_time: String,
}

/// Authentication result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResult {
    pub success: bool,
    pub profile: Option<UserProfile>,
    pub error: Option<String>,
}

/// Version list result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionListResult {
    pub versions: Vec<MinecraftVersion>,
    pub error: Option<String>,
}

/// Launch configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchConfig {
    pub version: String,
    pub game_directory: String,
    pub java_path: Option<String>,
    pub memory_mb: u32,
    pub username: String,
    pub uuid: String,
    pub access_token: String,
    pub window_width: u32,
    pub window_height: u32,
    pub fullscreen: bool,
}

/// Launch result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchResult {
    pub success: bool,
    pub pid: Option<u32>,
    pub error: Option<String>,
}

/// Installation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallResult {
    pub success: bool,
    pub error: Option<String>,
}

/// Java distribution info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JavaInfo {
    pub path: String,
    pub version: String,
    pub major_version: u8,
}

/// Launcher settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LauncherSettings {
    pub qualifier: String,
    pub organization: String,
    pub application: String,
    pub game_directory: String,
    pub java_directory: String,
}

impl Default for LauncherSettings {
    fn default() -> Self {
        Self {
            qualifier: "com".to_string(),
            organization: "ShardXL".to_string(),
            application: "Launcher".to_string(),
            game_directory: String::new(),
            java_directory: String::new(),
        }
    }
}

//! Configuration management for the engine
//!
//! Loads settings from JSON files to configure window size, rendering options, etc.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Main engine configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineConfig {
    /// Window configuration
    pub window: WindowConfig,
    /// Renderer configuration
    pub renderer: RendererConfig,
    /// Audio configuration
    pub audio: AudioConfig,
}

/// Window configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowConfig {
    /// Window title
    pub title: String,
    /// Window width in pixels
    pub width: u32,
    /// Window height in pixels
    pub height: u32,
    /// Whether to start in fullscreen mode
    pub fullscreen: bool,
    /// Whether the window is resizable
    pub resizable: bool,
    /// VSync enabled
    pub vsync: bool,
}

/// Renderer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RendererConfig {
    /// Maximum frames per second (0 = unlimited)
    pub target_fps: u32,
    /// Enable MSAA (multisampling anti-aliasing)
    pub msaa_samples: u32,
    /// Field of view in degrees
    pub fov: f32,
    /// Near clipping plane
    pub near_plane: f32,
    /// Far clipping plane
    pub far_plane: f32,
}

/// Audio configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfig {
    /// Master volume (0.0 to 1.0)
    pub master_volume: f32,
    /// Music volume (0.0 to 1.0)
    pub music_volume: f32,
    /// Sound effects volume (0.0 to 1.0)
    pub sfx_volume: f32,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            window: WindowConfig {
                title: "My Engine Game".to_string(),
                width: 1280,
                height: 720,
                fullscreen: false,
                resizable: true,
                vsync: true,
            },
            renderer: RendererConfig {
                target_fps: 60,
                msaa_samples: 4,
                fov: 70.0,
                near_plane: 0.1,
                far_plane: 1000.0,
            },
            audio: AudioConfig {
                master_volume: 1.0,
                music_volume: 0.8,
                sfx_volume: 1.0,
            },
        }
    }
}

impl EngineConfig {
    /// Load configuration from a JSON file
    ///
    /// # Arguments
    /// * `path` - Path to the JSON configuration file
    ///
    /// # Returns
    /// * `Ok(EngineConfig)` if successful
    /// * `Err(String)` if loading fails
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read config file: {}", e))?;
        
        let config: EngineConfig = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse config JSON: {}", e))?;
        
        Ok(config)
    }

    /// Save configuration to a JSON file
    ///
    /// # Arguments
    /// * `path` - Path to save the JSON configuration file
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), String> {
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;
        
        fs::write(path, content)
            .map_err(|e| format!("Failed to write config file: {}", e))?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = EngineConfig::default();
        assert_eq!(config.window.width, 1280);
        assert_eq!(config.window.height, 720);
        assert_eq!(config.renderer.target_fps, 60);
    }
}

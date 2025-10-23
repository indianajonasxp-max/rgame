//! # My Engine
//!
//! A modular Rust game engine library with Vulkan backend via wgpu.
//!
//! ## Features
//! - Modern GPU rendering via wgpu (Vulkan, DirectX 12, Metal)
//! - Window management and input handling via winit
//! - Audio playback via rodio
//! - Math utilities via glam
//! - Simple ECS (Entity Component System)
//! - Resource management for textures, shaders, and meshes
//! - 2D and 3D rendering capabilities
//! - Configuration loading from JSON
//! - Built-in logging and debug overlay
//!
//! ## Example Usage
//! ```no_run
//! use my_engine::prelude::*;
//!
//! fn main() {
//!     let config = EngineConfig::load("settings.json").unwrap_or_default();
//!     let mut engine = Engine::new(config);
//!     
//!     engine.run(|scene, input, delta| {
//!         // Your game logic here
//!         true // Return false to exit
//!     });
//! }
//! ```

pub mod audio;
pub mod config;
pub mod ecs;
pub mod engine;
pub mod input;
pub mod math;
pub mod renderer;
pub mod resource;
pub mod time;
pub mod utils;
pub mod window;

/// Commonly used types and traits
pub mod prelude {
    pub use crate::audio::{AudioManager, AudioSource};
    pub use crate::config::EngineConfig;
    pub use crate::ecs::{Component, Entity, EntityId, Scene};
    pub use crate::engine::Engine;
    pub use crate::input::{InputManager, Key, MouseButton};
    pub use crate::math::*;
    pub use crate::renderer::{Camera, Color, Renderer, Vertex};
    pub use crate::resource::{ResourceManager, Texture, Mesh, MeshBuilder};
    pub use crate::time::TimeManager;
    pub use crate::utils::{Random, Timer};
    pub use crate::window::Window;
    pub use glam::{Vec2, Vec3, Vec4, Mat4, Quat};
}

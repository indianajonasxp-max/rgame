# Changelog

All notable changes to My Engine will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2024-10-22

### Added
- **Core Engine Systems**
  - Main engine orchestrator with game loop
  - Event loop integration with winit
  - Delta time tracking and FPS counter
  
- **Rendering System**
  - wgpu-based renderer with Vulkan/DX12/Metal backend
  - 3D camera with perspective projection
  - Default WGSL shader with basic lighting
  - Color system with common color constants
  - Vertex format with position, texture coords, normals, and color
  
- **Entity Component System (ECS)**
  - Scene management
  - Entity creation and lifecycle
  - Component system with type-safe queries
  - Built-in Transform and Transform2D components
  
- **Resource Management**
  - Texture loading from files (PNG, JPG, etc.)
  - Mesh creation and GPU buffer management
  - Handle-based resource access
  - Built-in mesh builders (quad, cube)
  
- **Input System**
  - Keyboard state tracking (pressed, just_pressed, just_released)
  - Mouse button and position tracking
  - Scroll wheel support
  - Axis helpers for common input patterns (WASD/arrows)
  
- **Audio System**
  - Sound effect playback via rodio
  - Background music with looping support
  - Volume control (master, music, SFX)
  
- **Configuration**
  - JSON-based configuration files
  - Window, renderer, and audio settings
  - Save and load configuration
  
- **Math Utilities**
  - Re-export of glam types (Vec2, Vec3, Mat4, Quat)
  - Transform component for 3D objects
  - Transform2D for 2D games
  - Rectangle for 2D collision
  - Helper functions (deg_to_rad, lerp, etc.)
  
- **Utility Systems**
  - Random number generation (LCG-based)
  - Timer system for game events
  - Easing functions (ease-in, ease-out, bounce, elastic)
  - Color utilities (HSV conversion, lerp, rainbow)
  - Path utilities for asset management
  - Profiling helpers
  
- **Examples**
  - `basic_window`: Minimal setup and input handling
  - `spinning_cube`: 3D rendering with rotation
  - `ecs_demo`: Entity Component System usage
  - `advanced_game`: Complete game example with custom components
  - `camera_flight`: Camera animation demonstration
  - `stress_test`: Performance testing with many entities
  
- **Documentation**
  - Comprehensive README with quick start guide
  - Architecture documentation
  - Contributing guidelines
  - API documentation with examples
  - MIT License

### Technical Details
- Rust edition 2021
- wgpu 0.20 for GPU abstraction
- winit 0.29 for windowing
- rodio 0.18 for audio
- glam 0.27 for math
- serde 1.0 for serialization

### Known Limitations
- Single-threaded architecture
- No built-in physics system
- Basic lighting model only
- No UI/GUI system yet
- No animation system
- No networking support

### Future Plans
- Physics integration (rapier)
- Advanced lighting and shadows
- UI/GUI system
- Animation system
- Particle effects
- Scene serialization
- Asset hot-reloading
- Multi-threading support
- Networking capabilities

---

## Release Notes

### Version 0.1.0 - Initial Release

This is the first release of My Engine, a modular Rust game engine library. The focus of this release is providing a solid foundation with core systems that work well together.

**What's Working:**
✅ Window creation and management
✅ GPU rendering with wgpu
✅ Input handling (keyboard and mouse)
✅ Audio playback
✅ Simple but functional ECS
✅ Resource management
✅ Configuration system
✅ Comprehensive examples

**Performance:**
- Handles 1000+ entities at 60+ FPS
- Minimal overhead from engine systems
- GPU-accelerated rendering
- Efficient resource caching

**Ease of Use:**
- Simple API with sensible defaults
- Well-documented with examples
- Type-safe component system
- Panic-free library code

**Target Audience:**
- Indie game developers
- Game engine learners
- Rust gamedev enthusiasts
- Prototyping and game jams

**Getting Started:**
```rust
use my_engine::prelude::*;

fn main() {
    let config = EngineConfig::default();
    let engine = Engine::new(config);
    
    engine.run(|scene, input, delta| {
        // Your game here!
        !input.key_pressed(Key::Escape)
    });
}
```

**Feedback Welcome:**
This is an initial release and feedback is highly appreciated. Please open issues for bugs, feature requests, or questions!

---

[0.1.0]: https://github.com/yourusername/my_engine/releases/tag/v0.1.0

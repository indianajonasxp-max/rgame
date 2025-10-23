# My Engine - Rust Game Engine Library

A modular, well-documented Rust game engine library with Vulkan backend for 2D and 3D game development.

## Features

- **Modern GPU Rendering**: Vulkan/DirectX 12/Metal support via `wgpu`
- **Cross-Platform Windowing**: Window creation and event handling via `winit`
- **Audio System**: Sound effects and music playback via `rodio`
- **Math Library**: Comprehensive math support via `glam` (vectors, matrices, quaternions)
- **ECS Architecture**: Simple but powerful Entity Component System
- **Resource Management**: Efficient loading and caching of textures, meshes, and assets
- **2D & 3D Rendering**: Support for both 2D and 3D graphics with customizable pipelines
- **Configuration System**: JSON-based configuration loading
- **Debug Features**: Built-in FPS counter and logging system
- **Input Handling**: Keyboard, mouse, and gamepad support

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
my_engine = { path = "../my_engine" }
```

Or if you're in a workspace:

```toml
[dependencies]
my_engine = { path = "path/to/my_engine" }
```

## Quick Start

### Basic Window Example

```rust
use my_engine::prelude::*;

fn main() {
    let config = EngineConfig::default();
    let engine = Engine::new(config);

    engine.run(|scene, input, delta| {
        // Your game logic here
        
        if input.key_pressed(Key::Escape) {
            return false; // Exit
        }
        
        true // Continue running
    });
}
```

### 3D Cube Example

```rust
use my_engine::prelude::*;

fn main() {
    let config = EngineConfig::load("settings.json")
        .unwrap_or_default();
    
    let mut engine = Engine::new(config);
    
    // Create a cube entity
    let scene = engine.scene_mut();
    let cube = scene.create_entity("Cube".to_string());
    
    if let Some(entity) = scene.get_entity_mut(cube) {
        entity.add_component(Transform::new());
    }
    
    // Create and load cube mesh
    let cube_mesh = MeshBuilder::cube(2.0);
    let cube_handle = engine.resource_manager_mut().add_mesh(
        "cube".to_string(),
        cube_mesh,
        engine.renderer().unwrap().device(),
    );
    
    let mut rotation = 0.0;
    
    engine.run(move |scene, input, delta| {
        rotation += delta;
        
        // Update cube rotation
        if let Some(entity) = scene.get_entity_mut(cube) {
            if let Some(transform) = entity.get_component_mut::<Transform>() {
                transform.rotation = Quat::from_rotation_y(rotation);
            }
        }
        
        !input.key_pressed(Key::Escape)
    });
}
```

## Core Modules

### Engine
The main orchestrator that manages all subsystems:
- `Engine::new(config)` - Create a new engine instance
- `Engine::run(callback)` - Run the main game loop

### Scene & ECS
Organize your game objects using the Entity Component System:

```rust
let scene = engine.scene_mut();

// Create entity
let entity_id = scene.create_entity("MyEntity".to_string());

// Add components
if let Some(entity) = scene.get_entity_mut(entity_id) {
    entity.add_component(Transform::new());
    entity.add_component(MyCustomComponent { /* ... */ });
}

// Query entities
for entity in scene.active_entities() {
    if let Some(transform) = entity.get_component::<Transform>() {
        // Do something with transform
    }
}
```

### Renderer
GPU-accelerated rendering system:

```rust
let renderer = engine.renderer_mut().unwrap();

// Set clear color
renderer.set_clear_color(Color::rgb(0.1, 0.2, 0.3));

// Access camera
let camera = renderer.camera_mut();
camera.position = Vec3::new(0.0, 5.0, 10.0);
camera.target = Vec3::ZERO;
```

### Input
Handle keyboard and mouse input:

```rust
// Keyboard
if input.key_pressed(Key::KeyW) {
    // W is held down
}
if input.key_just_pressed(Key::Space) {
    // Space was just pressed this frame
}

// Mouse
if input.mouse_button_pressed(MouseButton::Left) {
    let pos = input.mouse_position();
    println!("Mouse at: {:?}", pos);
}

// Axis input
let horizontal = input.axis_horizontal(); // -1 to 1 (A/D or arrows)
let vertical = input.axis_vertical();     // -1 to 1 (W/S or arrows)
```

### Audio
Play sound effects and music:

```rust
let audio = engine.audio_mut();

// Load audio
let sfx = AudioSource::load("assets/sounds/jump.wav").unwrap();

// Play sound effect
audio.play_sfx(&sfx).ok();

// Play background music
let music = AudioSource::load("assets/music/theme.ogg").unwrap();
audio.play_music(&music, true).ok(); // true = loop

// Volume control
audio.set_master_volume(0.8);
audio.set_music_volume(0.6);
```

### Resource Management
Load and manage game assets:

```rust
let resources = engine.resource_manager_mut();
let device = engine.renderer().unwrap().device();
let queue = engine.renderer().unwrap().queue();

// Load texture
let texture_handle = resources.load_texture(
    "player_texture".to_string(),
    "assets/textures/player.png",
    device,
    queue,
).unwrap();

// Create mesh
let quad = MeshBuilder::quad(1.0, 1.0);
let mesh_handle = resources.add_mesh(
    "quad".to_string(),
    quad,
    device,
);

// Use resources
let texture = resources.get_texture(texture_handle).unwrap();
let mesh = resources.get_mesh(mesh_handle).unwrap();
```

### Time
Track frame time and delta time:

```rust
let time = engine.time();

let delta = time.delta_time();        // Seconds since last frame
let fps = time.fps();                 // Current frames per second
let elapsed = time.elapsed_secs();    // Total time since start
```

## Configuration

Create a `settings.json` file:

```json
{
  "window": {
    "title": "My Game",
    "width": 1920,
    "height": 1080,
    "fullscreen": false,
    "resizable": true,
    "vsync": true
  },
  "renderer": {
    "target_fps": 60,
    "msaa_samples": 4,
    "fov": 70.0,
    "near_plane": 0.1,
    "far_plane": 1000.0
  },
  "audio": {
    "master_volume": 1.0,
    "music_volume": 0.8,
    "sfx_volume": 1.0
  }
}
```

Load it:
```rust
let config = EngineConfig::load("settings.json").unwrap();
```

## Custom Components

Create your own components for the ECS:

```rust
use my_engine::ecs::Component;

#[derive(Debug)]
struct Velocity {
    x: f32,
    y: f32,
    z: f32,
}

impl Component for Velocity {}

// Use it
entity.add_component(Velocity { x: 1.0, y: 0.0, z: 0.0 });
```

## Examples

Run the included examples:

```bash
# Basic window and input
cargo run --example basic_window

# 3D spinning cube
cargo run --example spinning_cube

# ECS system demonstration
cargo run --example ecs_demo
```

## Architecture

```
my_engine/
├── src/
│   ├── lib.rs              # Main library entry point
│   ├── engine.rs           # Engine orchestration
│   ├── window.rs           # Window management
│   ├── renderer.rs         # GPU rendering system
│   ├── input.rs            # Input handling
│   ├── audio.rs            # Audio playback
│   ├── ecs.rs              # Entity Component System
│   ├── resource.rs         # Resource management
│   ├── time.rs             # Time and delta tracking
│   ├── config.rs           # Configuration loading
│   ├── math.rs             # Math utilities
│   └── shaders/
│       └── default.wgsl    # Default shader
├── examples/               # Example programs
└── Cargo.toml             # Dependencies
```

## Dependencies

- **wgpu** (0.20) - Modern GPU API abstraction
- **winit** (0.29) - Cross-platform windowing
- **rodio** (0.18) - Audio playback
- **glam** (0.27) - Math library
- **serde** (1.0) - Serialization
- **log** (0.4) - Logging facade

## Building

```bash
# Build the library
cargo build --release

# Build and run an example
cargo run --example basic_window --release

# Run tests
cargo test
```

## Performance Tips

1. **Use release mode** for final builds: `cargo build --release`
2. **Profile your game** using tools like `cargo flamegraph`
3. **Batch rendering calls** when possible
4. **Reuse resources** instead of loading them every frame
5. **Limit ECS queries** to only what you need

## Future Enhancements

Planned features for future versions:
- [ ] Physics system integration
- [ ] Particle system
- [ ] UI/GUI system
- [ ] Networking support
- [ ] Scene serialization
- [ ] Asset hot-reloading
- [ ] Advanced lighting and shadows
- [ ] Post-processing effects
- [ ] Animation system
- [ ] Gamepad support

## Contributing

This is a personal game engine library. Feel free to fork and modify for your own projects!

## License

MIT License - See LICENSE file for details

## Getting Help

- Check the examples in the `examples/` directory
- Read the API documentation: `cargo doc --open`
- Review the inline code comments

Happy game development! 🎮

# Quick Start Guide

Get up and running with My Engine in 5 minutes!

## Installation

### 1. Create a New Rust Project

```bash
cargo new my_game
cd my_game
```

### 2. Add My Engine as a Dependency

Edit `Cargo.toml`:

```toml
[dependencies]
my_engine = { path = "../my_engine" }
```

### 3. Write Your First Game

Replace the contents of `src/main.rs`:

```rust
use my_engine::prelude::*;

fn main() {
    // Create engine with default settings
    let config = EngineConfig::default();
    let engine = Engine::new(config);

    // Run the game loop
    engine.run(|scene, input, delta| {
        // Exit on Escape key
        if input.key_pressed(Key::Escape) {
            return false; // Stop the engine
        }

        true // Keep running
    });
}
```

### 4. Run Your Game

```bash
cargo run
```

You should see a window with a blue background!

## Adding Game Objects

Let's add a spinning cube:

```rust
use my_engine::prelude::*;

fn main() {
    let config = EngineConfig::default();
    let mut engine = Engine::new(config);

    // Create a cube entity
    let scene = engine.scene_mut();
    let cube_id = scene.create_entity("Cube".to_string());
    
    if let Some(entity) = scene.get_entity_mut(cube_id) {
        entity.add_component(Transform::new());
    }

    let mut rotation = 0.0;

    engine.run(move |scene, input, delta| {
        // Rotate the cube
        rotation += delta;
        
        if let Some(entity) = scene.get_entity_mut(cube_id) {
            if let Some(transform) = entity.get_component_mut::<Transform>() {
                transform.rotation = Quat::from_rotation_y(rotation);
            }
        }

        !input.key_pressed(Key::Escape)
    });
}
```

## Handling Input

```rust
engine.run(|scene, input, delta| {
    // Check if key is pressed
    if input.key_pressed(Key::KeyW) {
        println!("W is held down");
    }

    // Check if key was just pressed this frame
    if input.key_just_pressed(Key::Space) {
        println!("Space was just pressed!");
    }

    // Get axis input (-1 to 1)
    let horizontal = input.axis_horizontal(); // A/D or arrows
    let vertical = input.axis_vertical();     // W/S or arrows

    // Mouse input
    if input.mouse_button_pressed(MouseButton::Left) {
        let pos = input.mouse_position();
        println!("Mouse at: {:?}", pos);
    }

    true
});
```

## Creating Custom Components

```rust
use my_engine::prelude::*;
use my_engine::ecs::Component;

// Define your component
#[derive(Debug)]
struct Velocity {
    x: f32,
    y: f32,
    z: f32,
}

impl Component for Velocity {}

fn main() {
    let config = EngineConfig::default();
    let mut engine = Engine::new(config);

    // Create entity with custom component
    let scene = engine.scene_mut();
    let entity_id = scene.create_entity("MovingObject".to_string());
    
    if let Some(entity) = scene.get_entity_mut(entity_id) {
        entity.add_component(Transform::new());
        entity.add_component(Velocity { x: 1.0, y: 0.0, z: 0.0 });
    }

    engine.run(move |scene, input, delta| {
        // Update all entities with velocity
        for entity in scene.active_entities_mut() {
            if let (Some(transform), Some(velocity)) = (
                entity.get_component_mut::<Transform>(),
                entity.get_component::<Velocity>(),
            ) {
                transform.position.x += velocity.x * delta;
                transform.position.y += velocity.y * delta;
                transform.position.z += velocity.z * delta;
            }
        }

        !input.key_pressed(Key::Escape)
    });
}
```

## Using Configuration Files

Create `settings.json`:

```json
{
  "window": {
    "title": "My Awesome Game",
    "width": 1920,
    "height": 1080,
    "fullscreen": false,
    "resizable": true,
    "vsync": true
  },
  "renderer": {
    "target_fps": 144,
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
let config = EngineConfig::load("settings.json")
    .unwrap_or_else(|e| {
        eprintln!("Failed to load config: {}", e);
        EngineConfig::default()
    });
```

## Playing Audio

```rust
let mut engine = Engine::new(config);

// Load audio (in real code, handle errors properly)
let sfx = AudioSource::load("assets/sounds/jump.wav").unwrap();

engine.run(move |scene, input, delta| {
    // Play sound on spacebar
    if input.key_just_pressed(Key::Space) {
        engine.audio_mut().play_sfx(&sfx).ok();
    }

    true
});
```

## Next Steps

1. **Check out the examples**: `cargo run --example ecs_demo`
2. **Read the full README**: See `README.md` for comprehensive API docs
3. **Study the architecture**: See `ARCHITECTURE.md` to understand the engine
4. **Build something!**: Start prototyping your game idea

## Common Patterns

### Game State Management

```rust
struct GameState {
    score: u32,
    level: u32,
    paused: bool,
}

let mut state = GameState {
    score: 0,
    level: 1,
    paused: false,
};

engine.run(move |scene, input, delta| {
    if input.key_just_pressed(Key::KeyP) {
        state.paused = !state.paused;
    }

    if !state.paused {
        // Update game logic
        state.score += 1;
    }

    true
});
```

### Timers

```rust
let mut spawn_timer = Timer::repeating(2.0); // Every 2 seconds

engine.run(move |scene, input, delta| {
    if spawn_timer.update(delta) {
        // Spawn something every 2 seconds
        println!("Spawning!");
    }

    true
});
```

### Random Numbers

```rust
let mut rng = Random::from_time();

let x = rng.gen_range_f32(-10.0, 10.0);
let y = rng.gen_range_f32(0.0, 5.0);
```

## Troubleshooting

### "Failed to create surface"
- Make sure you have proper GPU drivers
- Try updating your graphics drivers

### Slow Performance
- Build in release mode: `cargo run --release`
- Reduce MSAA samples in config
- Disable VSync for higher FPS

### Audio Not Working
- Check that audio files exist
- Verify file format is supported (WAV, MP3, OGG, FLAC)
- Check volume settings

## Getting Help

- 📖 Read the full documentation
- 💻 Check the examples directory
- 🐛 Open an issue on GitHub
- 💬 Ask in discussions

Happy game development! 🎮

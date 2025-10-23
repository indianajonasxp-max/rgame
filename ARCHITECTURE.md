# My Engine Architecture

This document describes the architecture and design decisions of the my_engine library.

## Overview

My Engine is a modular game engine library built in Rust, designed to be:
- **Modular**: Each system is independent and can be used separately
- **Type-Safe**: Leveraging Rust's type system for compile-time safety
- **Performance-Oriented**: Zero-cost abstractions where possible
- **Easy to Use**: Simple API with sensible defaults

## Core Architecture

```
┌─────────────────────────────────────────────────────────┐
│                      Engine                             │
│  (Main orchestrator, owns all subsystems)               │
└─────────────────────────────────────────────────────────┘
                          │
        ┌─────────────────┼─────────────────┐
        ▼                 ▼                 ▼
   ┌────────┐      ┌──────────┐      ┌──────────┐
   │ Window │      │ Renderer │      │  Scene   │
   └────────┘      └──────────┘      └──────────┘
        │                │                  │
        │                │                  ├─► Entities
        │                │                  └─► Components
        ▼                ▼
   ┌────────┐      ┌──────────┐
   │  Input │      │ Resources│
   └────────┘      └──────────┘
                         │
                    ┌────┴────┐
                    ▼         ▼
              ┌─────────┐ ┌──────┐
              │Textures │ │Meshes│
              └─────────┘ └──────┘
```

## Module Breakdown

### Engine (`engine.rs`)
**Purpose**: Central coordinator that owns and manages all subsystems.

**Responsibilities**:
- Initialize all subsystems
- Run the main event loop
- Coordinate updates between systems
- Provide unified API to game developers

**Design Decision**: The engine owns all systems to ensure proper initialization order and cleanup.

### Window (`window.rs`)
**Purpose**: Window creation and management using winit.

**Responsibilities**:
- Create and configure the OS window
- Handle window events (resize, close, etc.)
- Provide window properties to other systems

**Why winit?**: Cross-platform, mature, and well-integrated with wgpu.

### Renderer (`renderer.rs`)
**Purpose**: GPU rendering using wgpu (Vulkan/DX12/Metal backend).

**Responsibilities**:
- Initialize GPU device and surface
- Manage render pipelines
- Handle camera and view matrices
- Execute draw calls

**Key Design Decisions**:
- **wgpu over raw Vulkan**: Portability and safety without sacrificing performance
- **Camera integrated**: Common need, simplifies API
- **Shader in WGSL**: Cross-platform shader language supported by wgpu

### ECS (`ecs.rs`)
**Purpose**: Entity Component System for game object organization.

**Architecture**:
```
Scene
  ├─ Entity (ID: 0)
  │   ├─ Transform Component
  │   ├─ Custom Component A
  │   └─ Custom Component B
  ├─ Entity (ID: 1)
  │   └─ Transform Component
  └─ ...
```

**Design Decisions**:
- **Simple implementation**: Easy to understand, no complex archetypes
- **Type-safe components**: Using Any trait for runtime type checking
- **HashMap-based storage**: Good performance for most games, simple to use
- **Component trait**: Allows user-defined components with zero boilerplate

**Trade-offs**:
- Not as performant as archetypal ECS (entt, bevy_ecs)
- Simpler to understand and use
- Sufficient for most indie games

### Resource Manager (`resource.rs`)
**Purpose**: Load and cache game assets.

**Responsibilities**:
- Load textures from files
- Create and manage meshes
- Provide GPU buffers for rendering
- Cache resources to avoid reloading

**Key Features**:
- **Handle-based access**: Type-safe handles prevent invalid access
- **Lazy GPU buffer creation**: Only create buffers when needed
- **Built-in primitives**: Quad and cube mesh builders

### Input (`input.rs`)
**Purpose**: Centralized input state management.

**Architecture**:
```
Frame N:   Key Down ──► keys_pressed
                    └─► keys_just_pressed

Frame N+1: Still Down ──► keys_pressed
                      └─► (cleared)

Frame N+2: Released ──► (removed)
                    └─► keys_just_released
```

**Design Decisions**:
- **State-based**: Query current state rather than event-based
- **Frame separation**: just_pressed vs pressed for one-shot actions
- **Axis helpers**: Common input patterns (WASD movement) built-in

### Audio (`audio.rs`)
**Purpose**: Sound effect and music playback.

**Responsibilities**:
- Initialize audio output stream
- Play sound effects (fire-and-forget)
- Play looping background music
- Volume control

**Why rodio?**: Simple API, cross-platform, supports common formats.

### Time (`time.rs`)
**Purpose**: Track frame timing and provide delta time.

**Responsibilities**:
- Calculate delta time between frames
- Track FPS
- Provide elapsed time since start

**Key Features**:
- **Automatic FPS calculation**: Updates every second
- **High precision**: Uses Instant for microsecond precision

### Config (`config.rs`)
**Purpose**: JSON-based configuration system.

**Benefits**:
- Easy to modify without recompiling
- Shareable settings
- Version control friendly

### Math (`math.rs`)
**Purpose**: Math utilities and common types.

**Design Decision**: 
- Re-export glam types rather than custom implementation
- glam is battle-tested, SIMD-optimized, and widely used
- Add convenience methods specific to game dev (Transform, Rect)

### Utils (`utils.rs`)
**Purpose**: Common utilities and helpers.

**Includes**:
- Random number generation (simple LCG)
- Timers for game events
- Easing functions for animations
- Color utilities
- Profiling helpers

## Data Flow

### Frame Update Flow

```
1. Event Loop Iteration
   └─► Process Window Events
       └─► Update Input State

2. Game Logic Callback
   └─► User code updates Scene
       ├─► Query entities
       ├─► Update components
       └─► Add/remove entities

3. Render
   ├─► Update Camera
   ├─► Build vertex buffers
   └─► Submit to GPU

4. Present Frame
   └─► Swap buffers

5. Update Time
   └─► Calculate delta time
   └─► Update FPS counter
```

### Resource Loading Flow

```
User Code
  └─► ResourceManager::load_texture()
       ├─► Load file from disk (image crate)
       ├─► Create GPU texture (wgpu)
       ├─► Store in cache
       └─► Return handle

Later...
  └─► ResourceManager::get_texture(handle)
       └─► Return reference from cache
```

## Performance Considerations

### Memory Management
- **Arena allocation**: ECS uses HashMap for simple allocation
- **Resource caching**: Textures/meshes loaded once, referenced by handles
- **GPU buffers**: Created once per mesh, reused for rendering

### Render Performance
- **Batch rendering**: Future enhancement for instanced rendering
- **GPU-side transforms**: Vertex shader transforms via uniform buffer
- **Simple pipeline**: Minimal state changes per frame

### ECS Performance
- **Linear iteration**: Scene iteration is cache-friendly
- **Component queries**: Type-based filtering via TypeId
- **Trade-off**: Simpler code vs. archetypal ECS performance

## Thread Safety

Currently **single-threaded** for simplicity:
- All systems run on main thread
- wgpu handles GPU threading internally
- rodio manages audio thread

**Future**: Could add multithreading for:
- Physics simulation
- Asset loading
- Particle systems

## Extension Points

The engine is designed to be extended:

1. **Custom Components**: Implement `Component` trait
2. **Custom Shaders**: Replace default.wgsl
3. **Custom Render Pipelines**: Create additional pipelines
4. **System Extension**: Add new modules alongside existing ones

## Design Patterns Used

- **Builder Pattern**: MeshBuilder for mesh creation
- **Resource Handle Pattern**: Type-safe asset references
- **Component Pattern**: ECS architecture
- **Strategy Pattern**: Game loop callback allows custom behavior
- **Facade Pattern**: Engine provides simplified API over complex subsystems

## Future Architecture Improvements

1. **Render Graph**: More flexible rendering pipeline
2. **Asset Pipeline**: Hot-reloading, compressed formats
3. **Multi-threading**: Parallel system updates
4. **Scripting**: Lua or WASM integration
5. **Networking**: Client-server architecture support
6. **Physics**: Integration with rapier or similar

## Code Quality Standards

- **Documentation**: All public APIs documented
- **Testing**: Unit tests for core functionality
- **Error Handling**: Result types for fallible operations
- **Logging**: Structured logging throughout
- **No Panics**: Avoid panic in library code, return errors instead

## Influences and Inspiration

This engine draws inspiration from:
- **Bevy**: ECS architecture and plugin system concepts
- **Piston**: Modular design philosophy
- **Unity**: Scene/Entity/Component structure
- **ggez**: Simplicity and beginner-friendliness

## Conclusion

My Engine prioritizes:
1. **Ease of Use** over maximum performance
2. **Clear Code** over clever optimizations
3. **Modularity** over monolithic design
4. **Safety** over raw speed

This makes it ideal for:
- Learning game development
- Prototyping game ideas
- Indie game projects
- Understanding game engine architecture

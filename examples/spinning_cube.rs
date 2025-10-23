//! Example: Spinning 3D Cube
//!
//! Demonstrates:
//! - 3D rendering
//! - Camera control
//! - Input handling
//! - Mesh creation
//! - ECS usage

use my_engine::prelude::*;

fn main() {
    // Load or create default configuration
    let config = EngineConfig::load("examples/settings.json").unwrap_or_else(|_| {
        let config = EngineConfig::default();
        config.save("examples/settings.json").ok();
        config
    });

    // Create engine
    let mut engine = Engine::new(config);

    // Create a scene and add a spinning cube entity
    let scene = engine.scene_mut();
    let cube_entity = scene.create_entity("SpinningCube".to_string());
    
    if let Some(entity) = scene.get_entity_mut(cube_entity) {
        let mut transform = Transform::new();
        transform.position = Vec3::new(0.0, 0.0, 0.0);
        entity.add_component(transform);
    }

    // Create cube mesh
    let cube_mesh = MeshBuilder::cube(2.0);
    let cube_handle = engine.resource_manager_mut().add_mesh(
        "cube".to_string(),
        cube_mesh,
        engine.renderer().unwrap().device(),
    );

    let mut rotation = 0.0f32;

    // Run the game loop
    engine.run(move |scene, input, delta| {
        // Update rotation
        rotation += delta * 1.0; // 1 radian per second

        // Update cube transform
        if let Some(entity) = scene.get_entity_mut(cube_entity) {
            if let Some(transform) = entity.get_component_mut::<Transform>() {
                transform.rotation = Quat::from_rotation_y(rotation) * Quat::from_rotation_x(rotation * 0.5);
            }
        }

        // Camera controls
        // (Camera control would be done through renderer in a more complete implementation)

        // Exit on Escape
        if input.key_pressed(Key::Escape) {
            return false;
        }

        true // Continue running
    });
}

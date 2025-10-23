//! Example: Camera Flight Demo
//!
//! Demonstrates:
//! - Smooth camera movement
//! - Math utilities
//! - Time-based animation
//! - Scene visualization

use my_engine::prelude::*;
use std::f32::consts::PI;

fn main() {
    println!("=== Camera Flight Demo ===");
    println!("Watch the camera fly around a central point!");
    println!("Press ESC to exit\n");

    let mut config = EngineConfig::default();
    config.window.title = "Camera Flight".to_string();
    
    let mut engine = Engine::new(config);

    // Create some reference objects in the scene
    let scene = engine.scene_mut();
    
    // Central sphere
    let center = scene.create_entity("Center".to_string());
    if let Some(entity) = scene.get_entity_mut(center) {
        entity.add_component(Transform::from_position(Vec3::ZERO));
    }

    // Ring of cubes
    for i in 0..8 {
        let angle = (i as f32 / 8.0) * 2.0 * PI;
        let radius = 5.0;
        let pos = Vec3::new(angle.cos() * radius, 0.0, angle.sin() * radius);
        
        let cube = scene.create_entity(format!("Cube_{}", i));
        if let Some(entity) = scene.get_entity_mut(cube) {
            entity.add_component(Transform::from_position(pos));
        }
    }

    println!("Scene created with {} entities", scene.entity_count());

    let mut time_elapsed = 0.0f32;

    engine.run(move |_scene, input, delta| {
        time_elapsed += delta;

        // Camera flies in a circular path
        let radius = 10.0;
        let height = 5.0 + (time_elapsed * 0.5).sin() * 3.0;
        let angle = time_elapsed * 0.5;

        let camera_pos = Vec3::new(
            angle.cos() * radius,
            height,
            angle.sin() * radius,
        );

        // Look at center with dynamic up vector
        let target = Vec3::ZERO;

        // Log camera position every few seconds
        if (time_elapsed % 5.0) < delta {
            println!("Camera position: ({:.2}, {:.2}, {:.2})", 
                camera_pos.x, camera_pos.y, camera_pos.z);
        }

        !input.key_pressed(Key::Escape)
    });
}

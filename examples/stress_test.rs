//! Example: Stress Test
//!
//! Performance test with many entities to demonstrate:
//! - ECS scalability
//! - Entity management
//! - Component queries
//! - Frame time tracking

use my_engine::prelude::*;
use my_engine::ecs::Component;

#[derive(Debug)]
struct Particle {
    velocity: Vec3,
    lifetime: f32,
}

impl Component for Particle {}

fn main() {
    println!("=== Engine Stress Test ===");
    println!("Creating many entities to test performance...\n");

    let config = EngineConfig::default();
    let mut engine = Engine::new(config);

    // Create many entities
    let num_entities = 1000;
    let scene = engine.scene_mut();

    for i in 0..num_entities {
        let entity_id = scene.create_entity(format!("Particle_{}", i));
        
        if let Some(entity) = scene.get_entity_mut(entity_id) {
            let angle = (i as f32 / num_entities as f32) * std::f32::consts::PI * 2.0;
            let radius = (i as f32 / num_entities as f32) * 10.0;
            
            let pos = Vec3::new(
                angle.cos() * radius,
                (i % 10) as f32,
                angle.sin() * radius,
            );

            let vel = Vec3::new(
                (angle + std::f32::consts::PI / 2.0).cos() * 2.0,
                0.0,
                (angle + std::f32::consts::PI / 2.0).sin() * 2.0,
            );

            entity.add_component(Transform::from_position(pos));
            entity.add_component(Particle {
                velocity: vel,
                lifetime: 10.0 + (i % 20) as f32,
            });
        }
    }

    println!("Created {} entities", scene.entity_count());

    let mut frame_count = 0;
    let mut min_fps = f32::MAX;
    let mut max_fps = 0.0f32;

    engine.run(move |scene, input, delta| {
        frame_count += 1;

        // Update all particles
        let mut expired_entities = Vec::new();

        for entity in scene.active_entities_mut() {
            if let Some(particle) = entity.get_component_mut::<Particle>() {
                particle.lifetime -= delta;

                if particle.lifetime <= 0.0 {
                    expired_entities.push(entity.id());
                    continue;
                }

                if let Some(transform) = entity.get_component_mut::<Transform>() {
                    transform.position += particle.velocity * delta;

                    // Bounce off boundaries
                    if transform.position.length() > 15.0 {
                        particle.velocity *= -0.9;
                    }
                }
            }
        }

        // Remove expired particles
        for id in expired_entities {
            scene.remove_entity(id);
        }

        // Track FPS statistics
        let fps = 1.0 / delta;
        if fps < min_fps {
            min_fps = fps;
        }
        if fps > max_fps {
            max_fps = fps;
        }

        // Print stats every 100 frames
        if frame_count % 100 == 0 {
            println!("Frame {}: {} entities, FPS: {:.1} (min: {:.1}, max: {:.1})",
                frame_count, scene.entity_count(), fps, min_fps, max_fps);
        }

        // Exit conditions
        if input.key_just_pressed(Key::Escape) || scene.entity_count() == 0 {
            println!("\n=== Final Statistics ===");
            println!("Total frames: {}", frame_count);
            println!("Min FPS: {:.1}", min_fps);
            println!("Max FPS: {:.1}", max_fps);
            println!("Remaining entities: {}", scene.entity_count());
            return false;
        }

        true
    });
}

//! Example: ECS System Demo
//!
//! Demonstrates:
//! - Entity creation
//! - Component management
//! - Scene queries
//! - Custom components

use my_engine::prelude::*;
use my_engine::ecs::Component;

// Custom component
#[derive(Debug)]
struct Velocity {
    x: f32,
    y: f32,
    z: f32,
}

impl Component for Velocity {}

#[derive(Debug)]
struct Health {
    current: f32,
    max: f32,
}

impl Component for Health {}

fn main() {
    let config = EngineConfig::default();
    let mut engine = Engine::new(config);

    // Create some entities
    {
        let scene = engine.scene_mut();

        // Create player entity
        let player = scene.create_entity("Player".to_string());
        if let Some(entity) = scene.get_entity_mut(player) {
            entity.add_component(Transform::from_position(Vec3::ZERO));
            entity.add_component(Velocity { x: 0.0, y: 0.0, z: 0.0 });
            entity.add_component(Health { current: 100.0, max: 100.0 });
        }

        // Create some enemies
        for i in 0..5 {
            let enemy = scene.create_entity(format!("Enemy_{}", i));
            if let Some(entity) = scene.get_entity_mut(enemy) {
                let pos = Vec3::new(i as f32 * 2.0, 0.0, -5.0);
                entity.add_component(Transform::from_position(pos));
                entity.add_component(Velocity { x: 0.0, y: 0.0, z: 1.0 });
                entity.add_component(Health { current: 50.0, max: 50.0 });
            }
        }

        println!("Created {} entities", scene.entity_count());
    }

    // Run game loop
    engine.run(|scene, input, delta| {
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

        // Find and update entities with health
        let mut entities_to_remove = Vec::new();
        for entity in scene.active_entities() {
            if let Some(health) = entity.get_component::<Health>() {
                if health.current <= 0.0 {
                    println!("{} has died!", entity.name());
                    entities_to_remove.push(entity.id());
                }
            }
        }

        // Remove dead entities
        for id in entities_to_remove {
            scene.remove_entity(id);
        }

        // Press Space to damage a random entity
        if input.key_just_pressed(Key::Space) {
            for entity in scene.active_entities_mut() {
                if let Some(health) = entity.get_component_mut::<Health>() {
                    health.current -= 25.0;
                    println!("{} took damage! Health: {}/{}", 
                        entity.name(), health.current, health.max);
                    break;
                }
            }
        }

        // Exit on Escape
        if input.key_just_pressed(Key::Escape) {
            return false;
        }

        true
    });
}

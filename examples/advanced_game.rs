//! Example: Advanced Game Demo
//!
//! A more complete game example demonstrating:
//! - Multiple entities with different behaviors
//! - Camera control
//! - Player movement
//! - Basic game state management
//! - Custom components

use my_engine::prelude::*;
use my_engine::ecs::Component;
use std::f32::consts::PI;

// Custom components
#[derive(Debug)]
struct Player {
    speed: f32,
    rotation_speed: f32,
}

impl Component for Player {}

#[derive(Debug)]
struct Enemy {
    speed: f32,
    patrol_radius: f32,
    angle: f32,
}

impl Component for Enemy {}

#[derive(Debug)]
struct Rotator {
    speed: f32,
}

impl Component for Rotator {}

fn main() {
    println!("=== Advanced Game Demo ===");
    println!("Controls:");
    println!("  WASD - Move camera");
    println!("  Arrow Keys - Move player");
    println!("  ESC - Exit");
    println!("========================\n");

    // Load configuration
    let mut config = EngineConfig::load("examples/settings.json")
        .unwrap_or_default();
    
    config.window.title = "Advanced Game Demo".to_string();

    // Create engine
    let mut engine = Engine::new(config);

    // Setup scene
    setup_scene(&mut engine);

    // Game state
    let mut camera_angle = 0.0f32;
    let mut camera_distance = 15.0f32;
    let mut camera_height = 8.0f32;

    // Run game loop
    engine.run(move |scene, input, delta| {
        // === Camera Controls ===
        
        // Rotate camera with A/D
        if input.key_pressed(Key::KeyA) {
            camera_angle += delta * 2.0;
        }
        if input.key_pressed(Key::KeyD) {
            camera_angle -= delta * 2.0;
        }

        // Zoom with W/S
        if input.key_pressed(Key::KeyW) {
            camera_distance = (camera_distance - delta * 5.0).max(5.0);
        }
        if input.key_pressed(Key::KeyS) {
            camera_distance = (camera_distance + delta * 5.0).min(30.0);
        }

        // Adjust height with Q/E
        if input.key_pressed(Key::KeyQ) {
            camera_height = (camera_height - delta * 5.0).max(2.0);
        }
        if input.key_pressed(Key::KeyE) {
            camera_height = (camera_height + delta * 5.0).min(20.0);
        }

        // === Player System ===
        update_player(scene, input, delta);

        // === Enemy System ===
        update_enemies(scene, delta);

        // === Rotator System ===
        update_rotators(scene, delta);

        // === Check Collisions ===
        check_collisions(scene);

        // Exit on Escape
        if input.key_just_pressed(Key::Escape) {
            println!("Exiting...");
            return false;
        }

        true
    });
}

fn setup_scene(engine: &mut Engine) {
    let scene = engine.scene_mut();

    // Create player
    let player_id = scene.create_entity("Player".to_string());
    if let Some(entity) = scene.get_entity_mut(player_id) {
        entity.add_component(Transform::from_position(Vec3::new(0.0, 0.5, 0.0)));
        entity.add_component(Player {
            speed: 5.0,
            rotation_speed: 3.0,
        });
    }
    println!("Created player entity");

    // Create enemies in a circle
    let num_enemies = 5;
    for i in 0..num_enemies {
        let angle = (i as f32 / num_enemies as f32) * 2.0 * PI;
        let radius = 8.0;
        let x = angle.cos() * radius;
        let z = angle.sin() * radius;

        let enemy_id = scene.create_entity(format!("Enemy_{}", i));
        if let Some(entity) = scene.get_entity_mut(enemy_id) {
            entity.add_component(Transform::from_position(Vec3::new(x, 0.5, z)));
            entity.add_component(Enemy {
                speed: 2.0,
                patrol_radius: radius,
                angle,
            });
        }
    }
    println!("Created {} enemies", num_enemies);

    // Create rotating platforms
    for i in 0..4 {
        let angle = (i as f32 / 4.0) * 2.0 * PI;
        let radius = 4.0;
        let x = angle.cos() * radius;
        let z = angle.sin() * radius;

        let platform_id = scene.create_entity(format!("Platform_{}", i));
        if let Some(entity) = scene.get_entity_mut(platform_id) {
            entity.add_component(Transform::from_position(Vec3::new(x, 0.0, z)));
            entity.add_component(Rotator {
                speed: 0.5 + i as f32 * 0.2,
            });
        }
    }
    println!("Created 4 rotating platforms");

    // Create ground plane entity
    let ground_id = scene.create_entity("Ground".to_string());
    if let Some(entity) = scene.get_entity_mut(ground_id) {
        let mut transform = Transform::new();
        transform.position = Vec3::new(0.0, -0.5, 0.0);
        transform.scale = Vec3::new(20.0, 0.1, 20.0);
        entity.add_component(transform);
    }
    println!("Created ground plane");

    println!("Scene setup complete! Total entities: {}", scene.entity_count());
}

fn update_player(scene: &mut Scene, input: &InputManager, delta: f32) {
    for entity in scene.active_entities_mut() {
        if let Some(player) = entity.get_component::<Player>() {
            let speed = player.speed;
            
            if let Some(transform) = entity.get_component_mut::<Transform>() {
                let mut movement = Vec3::ZERO;

                // Arrow key movement
                if input.key_pressed(Key::ArrowUp) {
                    movement.z -= 1.0;
                }
                if input.key_pressed(Key::ArrowDown) {
                    movement.z += 1.0;
                }
                if input.key_pressed(Key::ArrowLeft) {
                    movement.x -= 1.0;
                }
                if input.key_pressed(Key::ArrowRight) {
                    movement.x += 1.0;
                }

                // Normalize and apply speed
                if movement.length() > 0.0 {
                    movement = movement.normalize() * speed * delta;
                    transform.position += movement;

                    // Clamp to bounds
                    transform.position.x = transform.position.x.clamp(-10.0, 10.0);
                    transform.position.z = transform.position.z.clamp(-10.0, 10.0);

                    // Rotate to face movement direction
                    if movement.length() > 0.001 {
                        let target_rotation = (-movement.z).atan2(movement.x) - PI / 2.0;
                        transform.rotation = Quat::from_rotation_y(target_rotation);
                    }
                }
            }
        }
    }
}

fn update_enemies(scene: &mut Scene, delta: f32) {
    for entity in scene.active_entities_mut() {
        if let Some(enemy) = entity.get_component_mut::<Enemy>() {
            // Update patrol angle
            enemy.angle += enemy.speed * delta * 0.3;

            let x = enemy.angle.cos() * enemy.patrol_radius;
            let z = enemy.angle.sin() * enemy.patrol_radius;

            if let Some(transform) = entity.get_component_mut::<Transform>() {
                transform.position.x = x;
                transform.position.z = z;

                // Face movement direction
                transform.rotation = Quat::from_rotation_y(enemy.angle + PI / 2.0);
            }
        }
    }
}

fn update_rotators(scene: &mut Scene, delta: f32) {
    for entity in scene.active_entities_mut() {
        if let Some(rotator) = entity.get_component::<Rotator>() {
            if let Some(transform) = entity.get_component_mut::<Transform>() {
                let rotation = Quat::from_rotation_y(rotator.speed * delta);
                transform.rotation = rotation * transform.rotation;
            }
        }
    }
}

fn check_collisions(scene: &mut Scene) {
    // Simple collision detection between player and enemies
    let mut player_pos = None;

    // Get player position
    for entity in scene.active_entities() {
        if entity.has_component::<Player>() {
            if let Some(transform) = entity.get_component::<Transform>() {
                player_pos = Some(transform.position);
                break;
            }
        }
    }

    if let Some(player_pos) = player_pos {
        for entity in scene.active_entities() {
            if entity.has_component::<Enemy>() {
                if let Some(transform) = entity.get_component::<Transform>() {
                    let distance = (transform.position - player_pos).length();
                    if distance < 1.5 {
                        // Collision detected!
                        // In a real game, you'd handle this (damage, game over, etc.)
                    }
                }
            }
        }
    }
}

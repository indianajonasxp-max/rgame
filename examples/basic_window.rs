//! Example: Basic Window
//!
//! Demonstrates:
//! - Minimal engine setup
//! - Window creation
//! - Basic game loop
//! - Input handling

use my_engine::prelude::*;

fn main() {
    // Create default configuration
    let config = EngineConfig::default();

    // Create engine
    let engine = Engine::new(config);

    // Run the engine
    engine.run(|scene, input, delta| {
        // Check for escape key to exit
        if input.key_just_pressed(Key::Escape) {
            println!("Escape pressed - exiting");
            return false;
        }

        // Log mouse position when clicked
        if input.mouse_button_just_pressed(MouseButton::Left) {
            let pos = input.mouse_position();
            println!("Mouse clicked at: ({}, {})", pos.x, pos.y);
        }

        // Log keyboard input
        if input.key_just_pressed(Key::Space) {
            println!("Space pressed! Delta time: {:.4}s", delta);
        }

        // Movement example
        let horizontal = input.axis_horizontal();
        let vertical = input.axis_vertical();
        
        if horizontal != 0.0 || vertical != 0.0 {
            println!("Movement input: H={}, V={}", horizontal, vertical);
        }

        true // Continue running
    });
}

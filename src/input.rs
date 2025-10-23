//! Input handling for keyboard and mouse
//!
//! Tracks input state and provides query methods for game logic.

use std::collections::HashSet;
use winit::event::{ElementState, KeyEvent, MouseButton as WinitMouseButton};
use winit::keyboard::{KeyCode, PhysicalKey};
use glam::Vec2;

pub use winit::keyboard::KeyCode as Key;
pub use winit::event::MouseButton;

/// Manages input state for keyboard and mouse
#[derive(Debug)]
pub struct InputManager {
    // Keyboard state
    keys_pressed: HashSet<KeyCode>,
    keys_just_pressed: HashSet<KeyCode>,
    keys_just_released: HashSet<KeyCode>,

    // Mouse state
    mouse_buttons_pressed: HashSet<WinitMouseButton>,
    mouse_buttons_just_pressed: HashSet<WinitMouseButton>,
    mouse_buttons_just_released: HashSet<WinitMouseButton>,
    mouse_position: Vec2,
    mouse_delta: Vec2,
    scroll_delta: f32,
}

impl InputManager {
    /// Create a new input manager
    pub fn new() -> Self {
        Self {
            keys_pressed: HashSet::new(),
            keys_just_pressed: HashSet::new(),
            keys_just_released: HashSet::new(),
            mouse_buttons_pressed: HashSet::new(),
            mouse_buttons_just_pressed: HashSet::new(),
            mouse_buttons_just_released: HashSet::new(),
            mouse_position: Vec2::ZERO,
            mouse_delta: Vec2::ZERO,
            scroll_delta: 0.0,
        }
    }

    /// Update input state (call at the beginning of each frame)
    pub fn update(&mut self) {
        self.keys_just_pressed.clear();
        self.keys_just_released.clear();
        self.mouse_buttons_just_pressed.clear();
        self.mouse_buttons_just_released.clear();
        self.mouse_delta = Vec2::ZERO;
        self.scroll_delta = 0.0;
    }

    /// Handle keyboard input event
    pub fn handle_keyboard_input(&mut self, event: &KeyEvent) {
        if let PhysicalKey::Code(key_code) = event.physical_key {
            match event.state {
                ElementState::Pressed => {
                    if !self.keys_pressed.contains(&key_code) {
                        self.keys_just_pressed.insert(key_code);
                    }
                    self.keys_pressed.insert(key_code);
                }
                ElementState::Released => {
                    self.keys_pressed.remove(&key_code);
                    self.keys_just_released.insert(key_code);
                }
            }
        }
    }

    /// Handle mouse button input
    pub fn handle_mouse_button(&mut self, button: WinitMouseButton, state: ElementState) {
        match state {
            ElementState::Pressed => {
                if !self.mouse_buttons_pressed.contains(&button) {
                    self.mouse_buttons_just_pressed.insert(button);
                }
                self.mouse_buttons_pressed.insert(button);
            }
            ElementState::Released => {
                self.mouse_buttons_pressed.remove(&button);
                self.mouse_buttons_just_released.insert(button);
            }
        }
    }

    /// Handle mouse motion
    pub fn handle_mouse_motion(&mut self, delta: (f64, f64)) {
        self.mouse_delta = Vec2::new(delta.0 as f32, delta.1 as f32);
    }

    /// Set mouse position
    pub fn set_mouse_position(&mut self, position: Vec2) {
        self.mouse_position = position;
    }

    /// Handle mouse wheel scroll
    pub fn handle_scroll(&mut self, delta: f32) {
        self.scroll_delta = delta;
    }

    /// Check if a key is currently pressed
    pub fn key_pressed(&self, key: KeyCode) -> bool {
        self.keys_pressed.contains(&key)
    }

    /// Check if a key was just pressed this frame
    pub fn key_just_pressed(&self, key: KeyCode) -> bool {
        self.keys_just_pressed.contains(&key)
    }

    /// Check if a key was just released this frame
    pub fn key_just_released(&self, key: KeyCode) -> bool {
        self.keys_just_released.contains(&key)
    }

    /// Check if a mouse button is currently pressed
    pub fn mouse_button_pressed(&self, button: WinitMouseButton) -> bool {
        self.mouse_buttons_pressed.contains(&button)
    }

    /// Check if a mouse button was just pressed this frame
    pub fn mouse_button_just_pressed(&self, button: WinitMouseButton) -> bool {
        self.mouse_buttons_just_pressed.contains(&button)
    }

    /// Check if a mouse button was just released this frame
    pub fn mouse_button_just_released(&self, button: WinitMouseButton) -> bool {
        self.mouse_buttons_just_released.contains(&button)
    }

    /// Get current mouse position
    pub fn mouse_position(&self) -> Vec2 {
        self.mouse_position
    }

    /// Get mouse movement delta
    pub fn mouse_delta(&self) -> Vec2 {
        self.mouse_delta
    }

    /// Get scroll wheel delta
    pub fn scroll_delta(&self) -> f32 {
        self.scroll_delta
    }

    /// Get horizontal axis input (-1 to 1)
    pub fn axis_horizontal(&self) -> f32 {
        let mut value = 0.0;
        if self.key_pressed(KeyCode::ArrowLeft) || self.key_pressed(KeyCode::KeyA) {
            value -= 1.0;
        }
        if self.key_pressed(KeyCode::ArrowRight) || self.key_pressed(KeyCode::KeyD) {
            value += 1.0;
        }
        value
    }

    /// Get vertical axis input (-1 to 1)
    pub fn axis_vertical(&self) -> f32 {
        let mut value = 0.0;
        if self.key_pressed(KeyCode::ArrowDown) || self.key_pressed(KeyCode::KeyS) {
            value -= 1.0;
        }
        if self.key_pressed(KeyCode::ArrowUp) || self.key_pressed(KeyCode::KeyW) {
            value += 1.0;
        }
        value
    }
}

impl Default for InputManager {
    fn default() -> Self {
        Self::new()
    }
}

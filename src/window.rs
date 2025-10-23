//! Window management using winit
//!
//! Handles window creation, events, and surface management for rendering.

use winit::{
    event_loop::EventLoop,
    window::{Window as WinitWindow, WindowBuilder},
    dpi::PhysicalSize,
};
use crate::config::WindowConfig;

/// Window wrapper for the engine
pub struct Window {
    window: WinitWindow,
}

impl Window {
    /// Create a new window
    ///
    /// # Arguments
    /// * `config` - Window configuration
    /// * `event_loop` - The event loop (from winit)
    pub fn new(config: &WindowConfig, event_loop: &EventLoop<()>) -> Self {
        let mut window_builder = WindowBuilder::new()
            .with_title(&config.title)
            .with_inner_size(PhysicalSize::new(config.width, config.height))
            .with_resizable(config.resizable);

        if config.fullscreen {
            window_builder = window_builder.with_fullscreen(Some(
                winit::window::Fullscreen::Borderless(None)
            ));
        }

        let window = window_builder
            .build(event_loop)
            .expect("Failed to create window");

        log::info!("Window created: {}x{}", config.width, config.height);

        Self { window }
    }

    /// Get reference to the inner winit window
    pub fn inner(&self) -> &WinitWindow {
        &self.window
    }

    /// Get the window size
    pub fn size(&self) -> (u32, u32) {
        let size = self.window.inner_size();
        (size.width, size.height)
    }

    /// Get the window aspect ratio
    pub fn aspect_ratio(&self) -> f32 {
        let (width, height) = self.size();
        width as f32 / height as f32
    }

    /// Request a redraw
    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }

    /// Set window title
    pub fn set_title(&self, title: &str) {
        self.window.set_title(title);
    }

    /// Get window ID
    pub fn id(&self) -> winit::window::WindowId {
        self.window.id()
    }
}

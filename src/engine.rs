//! Main engine orchestration
//!
//! Brings together all engine systems and provides the main game loop.

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use crate::{
    audio::AudioManager,
    config::EngineConfig,
    ecs::Scene,
    input::InputManager,
    renderer::Renderer,
    resource::ResourceManager,
    time::TimeManager,
    window::Window,
};

/// Main engine struct that orchestrates all systems
pub struct Engine {
    config: EngineConfig,
    window: Option<Window>,
    renderer: Option<Renderer>,
    audio: AudioManager,
    input: InputManager,
    time: TimeManager,
    scene: Scene,
    resource_manager: ResourceManager,
    event_loop: Option<EventLoop<()>>,
    show_debug: bool,
}

impl Engine {
    /// Create a new engine with the given configuration
    pub fn new(config: EngineConfig) -> Self {
        // Initialize logging
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Info)
            .init();

        log::info!("Initializing My Engine...");

        // Create event loop
        let event_loop = EventLoop::new().expect("Failed to create event loop");

        // Create audio manager
        let audio = AudioManager::new().unwrap_or_else(|e| {
            log::warn!("Failed to initialize audio: {}", e);
            AudioManager::new().unwrap()
        });

        Self {
            config,
            window: None,
            renderer: None,
            audio,
            input: InputManager::new(),
            time: TimeManager::new(),
            scene: Scene::default(),
            resource_manager: ResourceManager::new(),
            event_loop: Some(event_loop),
            show_debug: true,
        }
    }

    /// Get reference to the scene
    pub fn scene(&self) -> &Scene {
        &self.scene
    }

    /// Get mutable reference to the scene
    pub fn scene_mut(&mut self) -> &mut Scene {
        &mut self.scene
    }

    /// Get reference to the resource manager
    pub fn resource_manager(&self) -> &ResourceManager {
        &self.resource_manager
    }

    /// Get mutable reference to the resource manager
    pub fn resource_manager_mut(&mut self) -> &mut ResourceManager {
        &mut self.resource_manager
    }

    /// Get reference to the renderer (if initialized)
    pub fn renderer(&self) -> Option<&Renderer> {
        self.renderer.as_ref()
    }

    /// Get mutable reference to the renderer (if initialized)
    pub fn renderer_mut(&mut self) -> Option<&mut Renderer> {
        self.renderer.as_mut()
    }

    /// Get reference to the audio manager
    pub fn audio(&self) -> &AudioManager {
        &self.audio
    }

    /// Get mutable reference to the audio manager
    pub fn audio_mut(&mut self) -> &mut AudioManager {
        &mut self.audio
    }

    /// Get reference to the input manager
    pub fn input(&self) -> &InputManager {
        &self.input
    }

    /// Get reference to the time manager
    pub fn time(&self) -> &TimeManager {
        &self.time
    }

    /// Toggle debug overlay
    pub fn set_show_debug(&mut self, show: bool) {
        self.show_debug = show;
    }

    /// Run the engine with a game loop callback
    ///
    /// The callback receives:
    /// - `scene`: Mutable reference to the scene
    /// - `input`: Reference to the input manager
    /// - `delta`: Delta time in seconds
    ///
    /// Return `true` to continue running, `false` to exit
    pub fn run<F>(mut self, mut game_loop: F)
    where
        F: FnMut(&mut Scene, &InputManager, f32) -> bool + 'static,
    {
        let event_loop = self.event_loop.take().expect("Event loop already consumed");

        // Create window
        let window = Window::new(&self.config.window, &event_loop);
        
        // Create renderer
        let renderer = pollster::block_on(Renderer::new(
            window.inner(),
            &self.config.renderer,
        ))
        .expect("Failed to create renderer");

        self.window = Some(window);
        self.renderer = Some(renderer);

        log::info!("Engine started!");

        let mut engine_state = self;

        event_loop.run(move |event, control_flow| {
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == engine_state.window.as_ref().unwrap().id() => {
                    match event {
                        WindowEvent::CloseRequested => {
                            log::info!("Window close requested");
                            control_flow.exit();
                        }
                        WindowEvent::Resized(physical_size) => {
                            if let Some(renderer) = &mut engine_state.renderer {
                                renderer.resize((physical_size.width, physical_size.height));
                            }
                        }
                        WindowEvent::KeyboardInput { event, .. } => {
                            engine_state.input.handle_keyboard_input(event);
                        }
                        WindowEvent::MouseInput { state, button, .. } => {
                            engine_state.input.handle_mouse_button(*button, *state);
                        }
                        WindowEvent::CursorMoved { position, .. } => {
                            engine_state.input.set_mouse_position(glam::Vec2::new(
                                position.x as f32,
                                position.y as f32,
                            ));
                        }
                        WindowEvent::MouseWheel { delta, .. } => {
                            let scroll = match delta {
                                MouseScrollDelta::LineDelta(_, y) => *y,
                                MouseScrollDelta::PixelDelta(pos) => pos.y as f32,
                            };
                            engine_state.input.handle_scroll(scroll);
                        }
                        WindowEvent::RedrawRequested => {
                            // Update time
                            engine_state.time.update();
                            let delta = engine_state.time.delta_time();

                            // Run game logic
                            let should_continue = game_loop(
                                &mut engine_state.scene,
                                &engine_state.input,
                                delta,
                            );

                            if !should_continue {
                                control_flow.exit();
                                return;
                            }

                            // Update camera
                            if let Some(renderer) = &mut engine_state.renderer {
                                renderer.update_camera();
                            }

                            // Update window title with FPS if debug is enabled
                            if engine_state.show_debug {
                                let fps = engine_state.time.fps();
                                let title = format!("{} - FPS: {:.0}", 
                                    engine_state.config.window.title, fps);
                                engine_state.window.as_ref().unwrap().set_title(&title);
                            }

                            // Update input for next frame
                            engine_state.input.update();
                        }
                        _ => {}
                    }
                }
                Event::AboutToWait => {
                    // Request redraw
                    if let Some(window) = &engine_state.window {
                        window.request_redraw();
                    }
                }
                _ => {}
            }
        }).expect("Event loop error");
    }
}

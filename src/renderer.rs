//! Rendering system using wgpu
//!
//! Provides a high-level rendering API with Vulkan backend.

use wgpu::util::DeviceExt;
use winit::window::Window;
use glam::{Mat4, Vec3};
use bytemuck::{Pod, Zeroable};
use crate::config::RendererConfig;

/// RGBA color
#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    /// Create a new color
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Create RGB color (alpha = 1.0)
    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self::new(r, g, b, 1.0)
    }

    /// Convert to wgpu color
    pub fn to_wgpu(&self) -> wgpu::Color {
        wgpu::Color {
            r: self.r as f64,
            g: self.g as f64,
            b: self.b as f64,
            a: self.a as f64,
        }
    }

    /// Convert to array
    pub fn to_array(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }

    // Common colors
    pub const WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const BLACK: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const RED: Color = Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const GREEN: Color = Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 };
    pub const BLUE: Color = Color { r: 0.0, g: 0.0, b: 1.0, a: 1.0 };
    pub const YELLOW: Color = Color { r: 1.0, g: 1.0, b: 0.0, a: 1.0 };
    pub const MAGENTA: Color = Color { r: 1.0, g: 0.0, b: 1.0, a: 1.0 };
    pub const CYAN: Color = Color { r: 0.0, g: 1.0, b: 1.0, a: 1.0 };
}

/// Vertex structure for rendering
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
    pub normal: [f32; 3],
    pub color: [f32; 4],
}

impl Vertex {
    /// Get vertex buffer layout
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                // Position
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // Tex coords
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                // Normal
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // Color
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}

/// Camera uniform buffer data
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

/// Camera for 3D rendering
pub struct Camera {
    pub position: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub fov: f32,
    pub aspect_ratio: f32,
    pub near: f32,
    pub far: f32,
}

impl Camera {
    /// Create a new camera
    pub fn new(position: Vec3, target: Vec3, aspect_ratio: f32) -> Self {
        Self {
            position,
            target,
            up: Vec3::Y,
            fov: 70.0,
            aspect_ratio,
            near: 0.1,
            far: 1000.0,
        }
    }

    /// Get the view matrix
    pub fn view_matrix(&self) -> Mat4 {
        Mat4::look_at_rh(self.position, self.target, self.up)
    }

    /// Get the projection matrix
    pub fn projection_matrix(&self) -> Mat4 {
        Mat4::perspective_rh(
            self.fov.to_radians(),
            self.aspect_ratio,
            self.near,
            self.far,
        )
    }

    /// Get combined view-projection matrix
    pub fn view_proj_matrix(&self) -> Mat4 {
        self.projection_matrix() * self.view_matrix()
    }

    /// Update aspect ratio (call when window resizes)
    pub fn update_aspect_ratio(&mut self, width: u32, height: u32) {
        self.aspect_ratio = width as f32 / height as f32;
    }
}

/// Main renderer
pub struct Renderer {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: (u32, u32),
    render_pipeline: wgpu::RenderPipeline,
    camera: Camera,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    clear_color: Color,
}

impl Renderer {
    /// Create a new renderer
    pub async fn new(window: &Window, renderer_config: &RendererConfig) -> Result<Self, String> {
        let size = window.inner_size();

        // Create wgpu instance
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        // Create surface
        // SAFETY: The window reference is valid for the lifetime of the surface
        let surface: wgpu::Surface<'static> = unsafe {
            let raw_surface = instance
                .create_surface(window)
                .map_err(|e| format!("Failed to create surface: {}", e))?;
            std::mem::transmute(raw_surface)
        };

        // Request adapter
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or("Failed to find suitable GPU adapter")?;

        log::info!("Using GPU: {}", adapter.get_info().name);

        // Request device and queue
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .map_err(|e| format!("Failed to create device: {}", e))?;

        // Configure surface
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: if renderer_config.target_fps == 0 {
                wgpu::PresentMode::AutoNoVsync
            } else {
                wgpu::PresentMode::AutoVsync
            },
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        // Create camera
        let camera = Camera::new(
            Vec3::new(0.0, 2.0, 5.0),
            Vec3::ZERO,
            size.width as f32 / size.height as f32,
        );

        // Create camera buffer
        let camera_uniform = CameraUniform {
            view_proj: camera.view_proj_matrix().to_cols_array_2d(),
        };

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Camera bind group layout
        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("camera_bind_group_layout"),
            });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        // Shader
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/default.wgsl").into()),
        });

        // Pipeline layout
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&camera_bind_group_layout],
                push_constant_ranges: &[],
            });

        // Render pipeline
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        log::info!("Renderer initialized: {}x{}", size.width, size.height);

        Ok(Self {
            surface,
            device,
            queue,
            config,
            size: (size.width, size.height),
            render_pipeline,
            camera,
            camera_buffer,
            camera_bind_group,
            clear_color: Color::new(0.1, 0.2, 0.3, 1.0),
        })
    }

    /// Get reference to the device
    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    /// Get reference to the queue
    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    /// Get mutable reference to the camera
    pub fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }

    /// Get reference to the camera
    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    /// Set clear color
    pub fn set_clear_color(&mut self, color: Color) {
        self.clear_color = color;
    }

    /// Resize the renderer
    pub fn resize(&mut self, new_size: (u32, u32)) {
        if new_size.0 > 0 && new_size.1 > 0 {
            self.size = new_size;
            self.config.width = new_size.0;
            self.config.height = new_size.1;
            self.surface.configure(&self.device, &self.config);
            self.camera.update_aspect_ratio(new_size.0, new_size.1);
            log::debug!("Resized to: {}x{}", new_size.0, new_size.1);
        }
    }

    /// Update camera uniform buffer
    pub fn update_camera(&mut self) {
        let camera_uniform = CameraUniform {
            view_proj: self.camera.view_proj_matrix().to_cols_array_2d(),
        };
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[camera_uniform]),
        );
    }

    /// Begin rendering a frame
    pub fn begin_frame(&self) -> Result<(wgpu::SurfaceTexture, wgpu::TextureView), String> {
        let output = self
            .surface
            .get_current_texture()
            .map_err(|e| format!("Failed to get surface texture: {}", e))?;

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        Ok((output, view))
    }

    /// Render a frame with the provided mesh data
    pub fn render(
        &mut self,
        vertex_buffer: &wgpu::Buffer,
        index_buffer: &wgpu::Buffer,
        num_indices: u32,
    ) -> Result<(), String> {
        let (output, view) = self.begin_frame()?;

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.clear_color.to_wgpu()),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(0..num_indices, 0, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    /// Get current size
    pub fn size(&self) -> (u32, u32) {
        self.size
    }
}

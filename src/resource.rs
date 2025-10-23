//! Resource management for textures, meshes, and other assets
//!
//! Provides loading and caching of game resources.

use std::collections::HashMap;
use std::path::Path;
use wgpu::{Device, Queue, TextureView};
use image::GenericImageView;
use crate::renderer::Vertex;

/// Handle to a loaded texture
pub type TextureHandle = usize;

/// Handle to a loaded mesh
pub type MeshHandle = usize;

/// A texture resource
pub struct Texture {
    pub view: TextureView,
    pub size: (u32, u32),
}

/// A mesh resource containing vertex and index data
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub vertex_buffer: Option<wgpu::Buffer>,
    pub index_buffer: Option<wgpu::Buffer>,
}

impl Mesh {
    /// Create a new mesh
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>) -> Self {
        Self {
            vertices,
            indices,
            vertex_buffer: None,
            index_buffer: None,
        }
    }

    /// Create GPU buffers for this mesh
    pub fn create_buffers(&mut self, device: &Device) {
        use wgpu::util::DeviceExt;

        self.vertex_buffer = Some(device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&self.vertices),
            usage: wgpu::BufferUsages::VERTEX,
        }));

        self.index_buffer = Some(device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&self.indices),
            usage: wgpu::BufferUsages::INDEX,
        }));
    }
}

/// Builder for creating meshes
pub struct MeshBuilder {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
}

impl MeshBuilder {
    /// Create a new mesh builder
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }

    /// Add a vertex
    pub fn add_vertex(mut self, vertex: Vertex) -> Self {
        self.vertices.push(vertex);
        self
    }

    /// Add vertices
    pub fn add_vertices(mut self, vertices: &[Vertex]) -> Self {
        self.vertices.extend_from_slice(vertices);
        self
    }

    /// Add an index
    pub fn add_index(mut self, index: u32) -> Self {
        self.indices.push(index);
        self
    }

    /// Add indices
    pub fn add_indices(mut self, indices: &[u32]) -> Self {
        self.indices.extend_from_slice(indices);
        self
    }

    /// Build the mesh
    pub fn build(self) -> Mesh {
        Mesh::new(self.vertices, self.indices)
    }

    /// Create a quad mesh (rectangle)
    pub fn quad(width: f32, height: f32) -> Mesh {
        let hw = width / 2.0;
        let hh = height / 2.0;

        let vertices = vec![
            Vertex { position: [-hw, -hh, 0.0], tex_coords: [0.0, 1.0], normal: [0.0, 0.0, 1.0], color: [1.0, 1.0, 1.0, 1.0] },
            Vertex { position: [hw, -hh, 0.0], tex_coords: [1.0, 1.0], normal: [0.0, 0.0, 1.0], color: [1.0, 1.0, 1.0, 1.0] },
            Vertex { position: [hw, hh, 0.0], tex_coords: [1.0, 0.0], normal: [0.0, 0.0, 1.0], color: [1.0, 1.0, 1.0, 1.0] },
            Vertex { position: [-hw, hh, 0.0], tex_coords: [0.0, 0.0], normal: [0.0, 0.0, 1.0], color: [1.0, 1.0, 1.0, 1.0] },
        ];

        let indices = vec![0, 1, 2, 0, 2, 3];

        Mesh::new(vertices, indices)
    }

    /// Create a cube mesh
    pub fn cube(size: f32) -> Mesh {
        let s = size / 2.0;
        let vertices = vec![
            // Front face
            Vertex { position: [-s, -s, s], tex_coords: [0.0, 1.0], normal: [0.0, 0.0, 1.0], color: [1.0, 1.0, 1.0, 1.0] },
            Vertex { position: [s, -s, s], tex_coords: [1.0, 1.0], normal: [0.0, 0.0, 1.0], color: [1.0, 1.0, 1.0, 1.0] },
            Vertex { position: [s, s, s], tex_coords: [1.0, 0.0], normal: [0.0, 0.0, 1.0], color: [1.0, 1.0, 1.0, 1.0] },
            Vertex { position: [-s, s, s], tex_coords: [0.0, 0.0], normal: [0.0, 0.0, 1.0], color: [1.0, 1.0, 1.0, 1.0] },
            // Back face
            Vertex { position: [s, -s, -s], tex_coords: [0.0, 1.0], normal: [0.0, 0.0, -1.0], color: [1.0, 1.0, 1.0, 1.0] },
            Vertex { position: [-s, -s, -s], tex_coords: [1.0, 1.0], normal: [0.0, 0.0, -1.0], color: [1.0, 1.0, 1.0, 1.0] },
            Vertex { position: [-s, s, -s], tex_coords: [1.0, 0.0], normal: [0.0, 0.0, -1.0], color: [1.0, 1.0, 1.0, 1.0] },
            Vertex { position: [s, s, -s], tex_coords: [0.0, 0.0], normal: [0.0, 0.0, -1.0], color: [1.0, 1.0, 1.0, 1.0] },
            // Top face
            Vertex { position: [-s, s, s], tex_coords: [0.0, 1.0], normal: [0.0, 1.0, 0.0], color: [1.0, 1.0, 1.0, 1.0] },
            Vertex { position: [s, s, s], tex_coords: [1.0, 1.0], normal: [0.0, 1.0, 0.0], color: [1.0, 1.0, 1.0, 1.0] },
            Vertex { position: [s, s, -s], tex_coords: [1.0, 0.0], normal: [0.0, 1.0, 0.0], color: [1.0, 1.0, 1.0, 1.0] },
            Vertex { position: [-s, s, -s], tex_coords: [0.0, 0.0], normal: [0.0, 1.0, 0.0], color: [1.0, 1.0, 1.0, 1.0] },
            // Bottom face
            Vertex { position: [-s, -s, -s], tex_coords: [0.0, 1.0], normal: [0.0, -1.0, 0.0], color: [1.0, 1.0, 1.0, 1.0] },
            Vertex { position: [s, -s, -s], tex_coords: [1.0, 1.0], normal: [0.0, -1.0, 0.0], color: [1.0, 1.0, 1.0, 1.0] },
            Vertex { position: [s, -s, s], tex_coords: [1.0, 0.0], normal: [0.0, -1.0, 0.0], color: [1.0, 1.0, 1.0, 1.0] },
            Vertex { position: [-s, -s, s], tex_coords: [0.0, 0.0], normal: [0.0, -1.0, 0.0], color: [1.0, 1.0, 1.0, 1.0] },
            // Right face
            Vertex { position: [s, -s, s], tex_coords: [0.0, 1.0], normal: [1.0, 0.0, 0.0], color: [1.0, 1.0, 1.0, 1.0] },
            Vertex { position: [s, -s, -s], tex_coords: [1.0, 1.0], normal: [1.0, 0.0, 0.0], color: [1.0, 1.0, 1.0, 1.0] },
            Vertex { position: [s, s, -s], tex_coords: [1.0, 0.0], normal: [1.0, 0.0, 0.0], color: [1.0, 1.0, 1.0, 1.0] },
            Vertex { position: [s, s, s], tex_coords: [0.0, 0.0], normal: [1.0, 0.0, 0.0], color: [1.0, 1.0, 1.0, 1.0] },
            // Left face
            Vertex { position: [-s, -s, -s], tex_coords: [0.0, 1.0], normal: [-1.0, 0.0, 0.0], color: [1.0, 1.0, 1.0, 1.0] },
            Vertex { position: [-s, -s, s], tex_coords: [1.0, 1.0], normal: [-1.0, 0.0, 0.0], color: [1.0, 1.0, 1.0, 1.0] },
            Vertex { position: [-s, s, s], tex_coords: [1.0, 0.0], normal: [-1.0, 0.0, 0.0], color: [1.0, 1.0, 1.0, 1.0] },
            Vertex { position: [-s, s, -s], tex_coords: [0.0, 0.0], normal: [-1.0, 0.0, 0.0], color: [1.0, 1.0, 1.0, 1.0] },
        ];

        let indices = vec![
            0, 1, 2, 0, 2, 3,       // Front
            4, 5, 6, 4, 6, 7,       // Back
            8, 9, 10, 8, 10, 11,    // Top
            12, 13, 14, 12, 14, 15, // Bottom
            16, 17, 18, 16, 18, 19, // Right
            20, 21, 22, 20, 22, 23, // Left
        ];

        Mesh::new(vertices, indices)
    }
}

impl Default for MeshBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Manages resources like textures and meshes
pub struct ResourceManager {
    textures: HashMap<String, Texture>,
    meshes: HashMap<String, Mesh>,
    texture_handles: Vec<String>,
    mesh_handles: Vec<String>,
}

impl ResourceManager {
    /// Create a new resource manager
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
            meshes: HashMap::new(),
            texture_handles: Vec::new(),
            mesh_handles: Vec::new(),
        }
    }

    /// Load a texture from a file
    pub fn load_texture<P: AsRef<Path>>(
        &mut self,
        name: String,
        path: P,
        device: &Device,
        queue: &Queue,
    ) -> Result<TextureHandle, String> {
        // Check if already loaded
        if let Some(index) = self.texture_handles.iter().position(|n| n == &name) {
            return Ok(index);
        }

        // Load image
        let img = image::open(path.as_ref())
            .map_err(|e| format!("Failed to load image: {}", e))?;
        let rgba = img.to_rgba8();
        let dimensions = img.dimensions();

        // Create texture
        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some(&name),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let texture_resource = Texture {
            view,
            size: dimensions,
        };

        self.textures.insert(name.clone(), texture_resource);
        self.texture_handles.push(name);

        log::info!("Loaded texture: {:?}", path.as_ref());
        Ok(self.texture_handles.len() - 1)
    }

    /// Get a texture by handle
    pub fn get_texture(&self, handle: TextureHandle) -> Option<&Texture> {
        let name = self.texture_handles.get(handle)?;
        self.textures.get(name)
    }

    /// Add a mesh to the resource manager
    pub fn add_mesh(&mut self, name: String, mut mesh: Mesh, device: &Device) -> MeshHandle {
        // Check if already exists
        if let Some(index) = self.mesh_handles.iter().position(|n| n == &name) {
            return index;
        }

        // Create GPU buffers
        mesh.create_buffers(device);

        self.meshes.insert(name.clone(), mesh);
        self.mesh_handles.push(name);

        log::info!("Added mesh");
        self.mesh_handles.len() - 1
    }

    /// Get a mesh by handle
    pub fn get_mesh(&self, handle: MeshHandle) -> Option<&Mesh> {
        let name = self.mesh_handles.get(handle)?;
        self.meshes.get(name)
    }

    /// Get a mutable mesh by handle
    pub fn get_mesh_mut(&mut self, handle: MeshHandle) -> Option<&mut Mesh> {
        let name = self.mesh_handles.get(handle)?;
        self.meshes.get_mut(name)
    }
}

impl Default for ResourceManager {
    fn default() -> Self {
        Self::new()
    }
}

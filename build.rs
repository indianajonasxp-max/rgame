// Build script for my_engine

fn main() {
    println!("cargo:rerun-if-changed=src/shaders/");
    
    // Validate shaders exist
    let shader_path = std::path::Path::new("src/shaders/default.wgsl");
    if !shader_path.exists() {
        panic!("Shader file not found: {:?}", shader_path);
    }
    
    println!("cargo:warning=Building My Engine library...");
}

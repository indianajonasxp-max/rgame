# Contributing to My Engine

Thank you for your interest in contributing to My Engine! This document provides guidelines and information for contributors.

## Code of Conduct

- Be respectful and constructive
- Focus on what is best for the community
- Show empathy towards other contributors

## Getting Started

1. **Fork the repository**
2. **Clone your fork**
   ```bash
   git clone https://github.com/yourusername/my_engine.git
   cd my_engine
   ```
3. **Create a branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

## Development Setup

### Prerequisites

- Rust 1.70 or later
- A GPU with Vulkan/DirectX 12/Metal support
- cargo and rustfmt

### Building

```bash
# Build the library
cargo build

# Build with optimizations
cargo build --release

# Run tests
cargo test

# Build documentation
cargo doc --open
```

### Running Examples

```bash
cargo run --example basic_window
cargo run --example spinning_cube
cargo run --example ecs_demo
```

## Project Structure

```
my_engine/
├── src/
│   ├── lib.rs           # Library entry point
│   ├── engine.rs        # Main engine orchestration
│   ├── renderer.rs      # GPU rendering
│   ├── ecs.rs           # Entity Component System
│   ├── audio.rs         # Audio playback
│   ├── input.rs         # Input handling
│   ├── window.rs        # Window management
│   ├── resource.rs      # Resource management
│   ├── time.rs          # Time tracking
│   ├── config.rs        # Configuration
│   ├── math.rs          # Math utilities
│   ├── utils.rs         # Utility functions
│   └── shaders/         # WGSL shaders
├── examples/            # Example programs
├── tests/               # Integration tests
└── benches/             # Benchmarks
```

## Coding Standards

### Style

- Follow Rust standard formatting (`cargo fmt`)
- Use `cargo clippy` and address warnings
- Keep functions focused and small
- Use meaningful variable names

### Documentation

- Document all public APIs with `///` comments
- Include examples in doc comments where helpful
- Use `//!` for module-level documentation
- Update README.md when adding major features

Example:
```rust
/// Calculates the distance between two points
///
/// # Arguments
/// * `a` - First point
/// * `b` - Second point
///
/// # Returns
/// The Euclidean distance as f32
///
/// # Example
/// ```
/// let dist = calculate_distance(Vec3::ZERO, Vec3::new(1.0, 0.0, 0.0));
/// assert_eq!(dist, 1.0);
/// ```
pub fn calculate_distance(a: Vec3, b: Vec3) -> f32 {
    (b - a).length()
}
```

### Error Handling

- Use `Result<T, String>` for operations that can fail
- Provide meaningful error messages
- Log errors appropriately
- Avoid panicking in library code

Example:
```rust
pub fn load_file(path: &str) -> Result<String, String> {
    std::fs::read_to_string(path)
        .map_err(|e| format!("Failed to load file {}: {}", path, e))
}
```

### Testing

- Write unit tests for new functionality
- Aim for high test coverage on critical paths
- Use integration tests for cross-module functionality

Example:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_matrix() {
        let transform = Transform::new();
        let matrix = transform.matrix();
        assert_eq!(matrix, Mat4::IDENTITY);
    }
}
```

## Contribution Types

### Bug Fixes

1. Create an issue describing the bug
2. Reference the issue in your PR
3. Include a test that reproduces the bug
4. Verify the fix resolves the issue

### New Features

1. Discuss the feature in an issue first
2. Keep changes focused and atomic
3. Update documentation
4. Add examples if appropriate
5. Write tests for the new functionality

### Documentation

- Fix typos and unclear explanations
- Add missing documentation
- Improve examples
- Update architecture docs

### Performance Improvements

- Provide benchmarks showing improvement
- Ensure correctness is maintained
- Document any trade-offs

## Pull Request Process

1. **Update your branch**
   ```bash
   git checkout main
   git pull upstream main
   git checkout your-branch
   git rebase main
   ```

2. **Run checks**
   ```bash
   cargo fmt --check
   cargo clippy
   cargo test
   cargo doc
   ```

3. **Commit your changes**
   - Use clear, descriptive commit messages
   - Reference issues: "Fixes #123"
   - Keep commits focused

4. **Push and create PR**
   ```bash
   git push origin your-branch
   ```
   Then create a pull request on GitHub

5. **PR Description**
   - Describe what changes you made
   - Explain why the change is needed
   - List any breaking changes
   - Include screenshots for visual changes

## Commit Message Guidelines

Format:
```
<type>: <short summary>

<detailed description if needed>

<footer>
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting)
- `refactor`: Code restructuring
- `perf`: Performance improvements
- `test`: Adding tests
- `chore`: Maintenance tasks

Example:
```
feat: Add particle system component

Implements a basic particle system for visual effects.
Includes emitter component and particle lifetime management.

Closes #45
```

## Code Review Process

- Be patient and respectful
- Address all review comments
- Ask questions if feedback is unclear
- Update based on suggestions
- Squash commits if requested

## Areas for Contribution

### High Priority

- [ ] Physics integration (rapier)
- [ ] UI/GUI system
- [ ] Particle effects
- [ ] Animation system
- [ ] More example games

### Medium Priority

- [ ] Scene serialization
- [ ] Asset hot-reloading
- [ ] Post-processing effects
- [ ] Shadow mapping
- [ ] Better documentation

### Good First Issues

- [ ] Add more primitive mesh builders (sphere, cylinder)
- [ ] Improve error messages
- [ ] Add more color constants
- [ ] Write more examples
- [ ] Fix documentation typos

## Questions?

- Open an issue for questions
- Check existing issues and PRs
- Read the architecture documentation

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

## Thank You!

Your contributions make this project better for everyone. We appreciate your time and effort! 🎮

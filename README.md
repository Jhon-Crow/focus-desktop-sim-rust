# Focus Desktop Simulator

A high-performance desktop simulator rewritten in Rust. This is a port of the original [focus-desktop-simulator](https://github.com/Jhon-Crow/focus-desktop-simulator) from JavaScript/Electron/Three.js to native Rust using wgpu for GPU rendering.

## Features

- Isometric 3D desk with interactive objects
- Drag and drop object manipulation
- Object rotation (scroll wheel) and scaling (shift + scroll)
- Multiple desk object types: coffee mug, laptop, notebook, plant, lamp, clock, and more
- State persistence (objects saved between sessions)
- High-performance native rendering with wgpu

## Requirements

- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- A GPU with Vulkan, Metal, or DX12 support

### Linux Additional Dependencies

```bash
# Ubuntu/Debian
sudo apt-get install -y libwayland-dev libxkbcommon-dev

# Fedora
sudo dnf install wayland-devel libxkbcommon-devel
```

## Building

```bash
# Clone the repository
git clone https://github.com/Jhon-Crow/focus-desktop-sim-in-fast-languages.git
cd focus-desktop-sim-in-fast-languages

# Build in release mode (recommended for performance)
cargo build --release

# Run the application
cargo run --release
```

## Controls

- **Left Click + Drag**: Move objects on the desk
- **Scroll Wheel**: Rotate selected object
- **Shift + Scroll**: Scale selected object
- **A Key**: Add a new coffee mug object

## Project Structure

```
src/
├── main.rs         # Application entry point, window, and rendering
├── camera.rs       # 3D camera with view/projection matrices
├── config.rs       # Configuration constants (desk size, colors, etc.)
├── desk_object.rs  # Object types and properties
├── physics.rs      # Physics engine for collision detection
├── state.rs        # State persistence (JSON)
└── shader.wgsl     # WGSL shader for 3D rendering
```

## Technology Stack

- **wgpu** - Modern GPU rendering API (WebGPU implementation)
- **winit** - Cross-platform window management
- **glam** - Fast math library for 3D graphics
- **serde** - Serialization for state persistence
- **bytemuck** - Safe byte casting for GPU buffers

## License

Unlicense - See LICENSE file for details.

## Credits

Original application: [focus-desktop-simulator](https://github.com/Jhon-Crow/focus-desktop-simulator) by Jhon-Crow

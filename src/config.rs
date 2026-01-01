//! Configuration module for the Focus Desktop Simulator
//!
//! Contains all configurable parameters for the application.

use glam::Vec3;

/// Camera configuration
pub struct CameraConfig {
    /// Field of view in degrees
    pub fov: f32,
    /// Near clipping plane
    pub near: f32,
    /// Far clipping plane
    pub far: f32,
    /// Initial camera position
    pub position: Vec3,
    /// Initial look-at target
    pub look_at: Vec3,
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            fov: 75.0,
            near: 0.1,
            far: 1000.0,
            position: Vec3::new(0.0, 4.5, 5.5),
            look_at: Vec3::new(0.0, 0.0, -1.5),
        }
    }
}

/// Desk configuration
pub struct DeskConfig {
    /// Width of the desk surface
    pub width: f32,
    /// Depth of the desk surface
    pub depth: f32,
    /// Thickness of the desk surface
    pub height: f32,
    /// Color of the desk (RGB hex)
    pub color: u32,
}

impl Default for DeskConfig {
    fn default() -> Self {
        Self {
            width: 10.0,
            depth: 7.0,
            height: 0.1,
            color: 0x8b6914,
        }
    }
}

/// Physics configuration
pub struct PhysicsConfig {
    /// Height objects lift when dragged
    pub lift_height: f32,
    /// Speed of object lifting
    pub lift_speed: f32,
    /// Speed of object dropping
    pub drop_speed: f32,
    /// Gravity constant
    pub gravity: f32,
    /// Friction coefficient
    pub friction: f32,
    /// Bounce factor for collisions
    pub bounce_factor: f32,
}

impl Default for PhysicsConfig {
    fn default() -> Self {
        Self {
            lift_height: 0.5,
            lift_speed: 0.15,
            drop_speed: 0.2,
            gravity: 0.02,
            friction: 0.85,
            bounce_factor: 0.4,
        }
    }
}

/// Color configuration
pub struct ColorConfig {
    /// Background color (RGB hex)
    pub background: u32,
    /// Ambient light color
    pub ambient: u32,
    /// Directional light color
    pub directional: u32,
    /// Ground/floor color
    pub ground: u32,
}

impl Default for ColorConfig {
    fn default() -> Self {
        Self {
            background: 0x1a1a2e,
            ambient: 0x404060,
            directional: 0xffffff,
            ground: 0x2d3748,
        }
    }
}

/// Pixelation effect configuration (Signalis-style)
pub struct PixelationConfig {
    /// Whether pixelation effect is enabled
    pub enabled: bool,
    /// Size of pixels (higher = more pixelated)
    pub pixel_size: u32,
    /// Edge detection strength based on normals
    pub normal_edge_strength: f32,
    /// Edge detection strength based on depth
    pub depth_edge_strength: f32,
}

impl Default for PixelationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            pixel_size: 4,
            normal_edge_strength: 0.3,
            depth_edge_strength: 0.4,
        }
    }
}

/// Main configuration struct containing all settings
pub struct Config {
    pub camera: CameraConfig,
    pub desk: DeskConfig,
    pub physics: PhysicsConfig,
    pub colors: ColorConfig,
    pub pixelation: PixelationConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            camera: CameraConfig::default(),
            desk: DeskConfig::default(),
            physics: PhysicsConfig::default(),
            colors: ColorConfig::default(),
            pixelation: PixelationConfig::default(),
        }
    }
}

/// Convert a hex color to RGB f32 tuple (0.0-1.0)
pub fn hex_to_rgb(hex: u32) -> (f32, f32, f32) {
    let r = ((hex >> 16) & 0xFF) as f32 / 255.0;
    let g = ((hex >> 8) & 0xFF) as f32 / 255.0;
    let b = (hex & 0xFF) as f32 / 255.0;
    (r, g, b)
}

/// Convert a hex color to RGBA f32 array [0.0-1.0]
pub fn hex_to_rgba(hex: u32) -> [f32; 4] {
    let (r, g, b) = hex_to_rgb(hex);
    [r, g, b, 1.0]
}

/// Global configuration instance
pub static CONFIG: std::sync::LazyLock<Config> = std::sync::LazyLock::new(Config::default);

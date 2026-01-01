// Focus Desktop Simulator - Main Shader
// WGSL shader for 3D rendering with basic lighting

// Camera uniform buffer
struct CameraUniform {
    view_proj: mat4x4<f32>,
    position: vec4<f32>,
}

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

// Vertex input
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) color: vec4<f32>,
}

// Vertex output / Fragment input
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) color: vec4<f32>,
}

// Vertex shader
@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    // Transform to clip space
    out.clip_position = camera.view_proj * vec4<f32>(in.position, 1.0);

    // Pass through world position and normal
    out.world_position = in.position;
    out.world_normal = in.normal;
    out.color = in.color;

    return out;
}

// Fragment shader with basic lighting
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Light direction (from top-right)
    let light_dir = normalize(vec3<f32>(0.5, 1.0, 0.3));

    // Ambient light
    let ambient_color = vec3<f32>(0.25, 0.25, 0.35);

    // Directional light
    let normal = normalize(in.world_normal);
    let diffuse = max(dot(normal, light_dir), 0.0);

    // Combine lighting
    let light = ambient_color + diffuse * vec3<f32>(0.8, 0.8, 0.75);

    // Apply lighting to base color
    let lit_color = in.color.rgb * light;

    // Simple fog effect based on distance from camera
    let dist = length(in.world_position - camera.position.xyz);
    let fog_factor = 1.0 - clamp((dist - 10.0) / 40.0, 0.0, 0.6);

    // Background/fog color
    let fog_color = vec3<f32>(0.1, 0.1, 0.18);

    let final_color = mix(fog_color, lit_color, fog_factor);

    return vec4<f32>(final_color, in.color.a);
}

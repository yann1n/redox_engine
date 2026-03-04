//! Built-in WGSL shaders and helper to create shader modules.
//!
//! In the MVP the shader source is embedded as a constant string.
//! This will be replaced by file-based loading in later milestones.

/// WGSL source for the default forward-shading shader.
///
/// ## Bind groups
/// - Group 0, Binding 0: `CameraUniform` (view_proj + camera_pos).
/// - Group 1, Binding 0: `LightUniform`  (direction, colour, ambient).
/// - Group 2, Binding 0: `ModelUniform`  (model matrix).
/// - Group 3, Binding 0: Texture.
/// - Group 3, Binding 1: Sampler.
pub const FORWARD_SHADER_SRC: &str = r#"
// ---- Uniforms ----

struct CameraUniform {
    view_proj: mat4x4<f32>,
    camera_pos: vec4<f32>,
};

struct LightUniform {
    direction:  vec4<f32>,
    color:      vec4<f32>,
    ambient:    vec4<f32>,
};

struct ModelUniform {
    model: mat4x4<f32>,
};

@group(0) @binding(0) var<uniform> camera: CameraUniform;
@group(1) @binding(0) var<uniform> light:  LightUniform;
@group(2) @binding(0) var<uniform> model_u: ModelUniform;
@group(3) @binding(0) var t_diffuse: texture_2d<f32>;
@group(3) @binding(1) var s_diffuse: sampler;

// ---- Vertex stage ----

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal:   vec3<f32>,
    @location(2) uv:       vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_normal: vec3<f32>,
    @location(1) uv:           vec2<f32>,
    @location(2) world_pos:    vec3<f32>,
};

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let world_pos = model_u.model * vec4<f32>(in.position, 1.0);
    out.clip_position = camera.view_proj * world_pos;
    // Normal transform (ignoring non-uniform scale for MVP).
    out.world_normal = normalize((model_u.model * vec4<f32>(in.normal, 0.0)).xyz);
    out.uv = in.uv;
    out.world_pos = world_pos.xyz;
    return out;
}

// ---- Fragment stage ----

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let tex_color = textureSample(t_diffuse, s_diffuse, in.uv);

    let n = normalize(in.world_normal);
    let l = normalize(light.direction.xyz);

    // Lambertian diffuse
    let diff = max(dot(n, l), 0.0);
    let diffuse = light.color.xyz * diff;

    let final_color = (light.ambient.xyz + diffuse) * tex_color.xyz;
    return vec4<f32>(final_color, tex_color.a);
}
"#;

/// Creates a `wgpu::ShaderModule` from WGSL source code.
pub fn create_shader_module(
    device: &wgpu::Device,
    label: &str,
    source: &str,
) -> wgpu::ShaderModule {
    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some(label),
        source: wgpu::ShaderSource::Wgsl(source.into()),
    })
}

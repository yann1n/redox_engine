//! Texture loading and GPU upload utilities.

/// A GPU texture with its view and sampler, ready to be bound in a shader.
pub struct Texture {
    #[allow(dead_code)]
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub width: u32,
    pub height: u32,
}

impl Texture {
    /// Decodes image bytes (PNG, JPEG, etc.) and uploads the result to the GPU.
    ///
    /// The texture is created in `Rgba8UnormSrgb` format.
    pub fn from_bytes(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bytes: &[u8],
        label: &str,
    ) -> Result<Self, image::ImageError> {
        let img = image::load_from_memory(bytes)?;
        let rgba = img.to_rgba8();
        let (width, height) = rgba.dimensions();

        let size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some(label),
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
                bytes_per_row: Some(4 * width),
                rows_per_image: Some(height),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some(&format!("{label}_sampler")),
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            address_mode_w: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Ok(Self {
            texture,
            view,
            sampler,
            width,
            height,
        })
    }

    /// Creates a 1×1 white texture (fallback for materials without a texture).
    pub fn white_1x1(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        Self::from_bytes(device, queue, &create_white_png(), "white_1x1")
            .expect("Failed to create 1x1 white fallback texture")
    }
}

/// Produces a minimal 1×1 white PNG in memory.
fn create_white_png() -> Vec<u8> {
    use image::ImageEncoder;
    let mut buf = std::io::Cursor::new(Vec::new());
    {
        let encoder = image::codecs::png::PngEncoder::new(&mut buf);
        encoder
            .write_image(&[255u8, 255, 255, 255], 1, 1, image::ColorType::Rgba8)
            .expect("PNG encode failed");
    }
    buf.into_inner()
}

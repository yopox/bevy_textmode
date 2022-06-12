use bevy::ecs::system::lifetimeless::SRes;
use bevy::ecs::system::SystemParamItem;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::render_asset::{PrepareAssetError, RenderAsset, RenderAssets};
use bevy::render::render_resource::{BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType, BufferBindingType, BufferInitDescriptor, BufferSize, BufferUsages, SamplerBindingType, ShaderStages, TextureSampleType, TextureViewDimension};
use bevy::render::render_resource::std140::{AsStd140, Std140};
use bevy::render::renderer::RenderDevice;
use bevy::sprite::{Material2d, Material2dPipeline};

#[derive(Debug, Clone, Component, TypeUuid)]
#[uuid = "eb3bfce5-5e0d-4a0e-bf7c-dec3e8a6d330"]
pub struct TileMaterial {
    pub(crate) texture: Handle<Image>,
    pub(crate) bg: Color,
    pub(crate) fg: Color,
}

#[derive(Clone)]
pub struct GpuTileMaterial {
    bind_group: BindGroup,
}

impl RenderAsset for TileMaterial {
    type ExtractedAsset = TileMaterial;
    type PreparedAsset = GpuTileMaterial;
    type Param = (
        SRes<RenderDevice>,
        SRes<RenderAssets<Image>>,
        SRes<Material2dPipeline<Self>>,
    );
    fn extract_asset(&self) -> Self::ExtractedAsset {
        self.clone()
    }

    fn prepare_asset(
        extracted_asset: Self::ExtractedAsset,
        (render_device, gpu_images, material_pipeline): &mut SystemParamItem<Self::Param>,
    ) -> Result<Self::PreparedAsset, PrepareAssetError<Self::ExtractedAsset>> {
        let gpu_image = match gpu_images.get(&extracted_asset.texture) {
            Some(gpu_image) => gpu_image,
            // if the image isn't loaded yet, try next frame
            None => return Err(PrepareAssetError::RetryNextUpdate(extracted_asset)),
        };

        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&gpu_image.texture_view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(&gpu_image.sampler),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: render_device.create_buffer_with_data(&BufferInitDescriptor {
                        contents: Vec4::from_slice(&extracted_asset.bg.as_linear_rgba_f32()).as_std140().as_bytes(),
                        label: None,
                        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
                    }).as_entire_binding()
                },
                BindGroupEntry {
                    binding: 3,
                    resource: render_device.create_buffer_with_data(&BufferInitDescriptor {
                        contents: Vec4::from_slice(&extracted_asset.fg.as_linear_rgba_f32()).as_std140().as_bytes(),
                        label: None,
                        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
                    }).as_entire_binding()
                },
            ],
            label: None,
            layout: &material_pipeline.material2d_layout,
        });

        Ok(GpuTileMaterial { bind_group })
    }
}

impl Material2d for TileMaterial {
    fn bind_group(render_asset: &<Self as RenderAsset>::PreparedAsset) -> &BindGroup {
        &render_asset.bind_group
    }

    fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
        render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: true },
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: BufferSize::new(Vec4::std140_size_static() as u64),
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 3,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: BufferSize::new(Vec4::std140_size_static() as u64),
                    },
                    count: None,
                }
            ],
            label: None,
        })
    }

    fn fragment_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        Some(asset_server.load("shader.wgsl"))
    }
}
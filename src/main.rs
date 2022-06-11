use bevy::ecs::system::lifetimeless::SRes;
use bevy::ecs::system::{Resource, SystemParamItem};
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::render_asset::{PrepareAssetError, RenderAsset, RenderAssets};
use bevy::render::render_resource::{BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType, BufferBindingType, BufferInitDescriptor, BufferSize, BufferUsages, Extent3d, SamplerBindingType, ShaderStages, TextureDimension, TextureFormat, TextureSampleType, TextureViewDimension};
use bevy::render::renderer::RenderDevice;
use bevy::sprite::{Material2d, Material2dPipeline, Material2dPlugin, MaterialMesh2dBundle, Mesh2dHandle};
use bevy::window::PresentMode;
use core::default::Default;
use std::borrow::BorrowMut;
use std::collections::HashMap;
use bevy::ecs::query::WorldQuery;
use bevy::render::render_resource::std140::{AsStd140, Std140};
use rand::random;
use image;
use image::{GenericImageView, Rgba};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(Material2dPlugin::<TileMaterial>::default())
        .insert_resource(WindowDescriptor {
            title: "bevy_textmode".to_string(),
            width: 1920.0,
            height: 1080.0,
            present_mode: PresentMode::Immediate,
            ..Default::default()
        })
        .add_startup_system(setup)
        .add_startup_stage(
            "game_setup_grid",
            SystemStage::single(spawn_grid),
        )
        .add_system(update_color)
        .run();
}

#[derive(Component)]
pub struct Tiles {
    tiles: HashMap<usize, Handle<Image>>
}

#[derive(Component)]
pub struct BasicMesh {
    tile: Handle<Mesh>
}

#[derive(Component, Copy, Clone, Eq, PartialEq)]
pub struct TilePos {
    pub x: i32,
    pub y: i32,
}

#[derive(Bundle, Clone)]
pub struct TextModeBundle {
    pub pos: TilePos,
    pub mesh: Mesh2dHandle,
    pub material: Handle<TileMaterial>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}

impl TextModeBundle {
    fn new(tiles: &Res<Tiles>, mut materials: &mut ResMut<Assets<TileMaterial>>, index: usize, x: i32, y: i32, bg: Color, fg: Color, mesh: Handle<Mesh>) -> Self {
        let texture = tiles.tiles.get(&index).expect("Couldn't find tile.");
        TextModeBundle {
            pos: TilePos { x, y },
            mesh: mesh.into(),
            material: materials.add(TileMaterial { texture: texture.clone(), bg, fg }),
            transform: Transform {
                translation: Vec3::new(x as f32 * 8.0, y as f32 * 8.0, 0.0),
                ..Default::default()
            },
            global_transform: Default::default(),
            visibility: Default::default(),
            computed_visibility: Default::default()
        }
    }
}

fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    init_spritesheet("assets/MRMOTEXT.png", 8, images, &mut commands);

    commands.insert_resource(BasicMesh {
        tile: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(8.0, 8.0))))
    });

    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform = Transform {
        scale: Vec3::new(0.25, 0.25, 1.0),
        ..Default::default()
    };
    commands.spawn_bundle(camera);
}

fn init_spritesheet(
    path: &str,
    size: u32,
    mut images: ResMut<Assets<Image>>,
    mut commands: &mut Commands,
) {
    let img = image::open(path).expect("File not found");
    let tileWidth = img.width() / size;
    let tileHeight = img.height() / size;
    let mut tilesVec = vec![];
    for i in 0..(tileWidth * tileHeight) {
        tilesVec.push(vec![]);
    }
    for pixel in img.pixels() {
        let x = pixel.0;
        let y = pixel.1;
        let n = x / size + y / size * tileWidth;
        if let Some(tile) = tilesVec.get_mut(n as usize) {
            match pixel.2.0 {
                [0, 0, 0, _] => {
                    tile.push(0);
                    tile.push(0);
                    tile.push(0);
                    tile.push(255);
                }
                _ => {
                    tile.push(255);
                    tile.push(255);
                    tile.push(255);
                    tile.push(255);
                }
            }
        }
    }

    let mut tiles_hm = HashMap::new();

    for i in 0..tilesVec.len() {
        let handle = images.add(Image::new(
            Extent3d {
                width: size,
                height: size,
                depth_or_array_layers: 1
            },
            TextureDimension::D2,
            tilesVec.get(i).unwrap().clone(),
            TextureFormat::Rgba8UnormSrgb
        ));
        tiles_hm.insert(i, handle);
    }

    commands.insert_resource(Tiles { tiles: tiles_hm });
}

fn spawn_grid(
    mut commands: Commands,
    mut materials: ResMut<Assets<TileMaterial>>,
    tiles: Res<Tiles>,
    meshes: Res<BasicMesh>,
) {
    for x in 0..8 {
        for y in 0..8 {
            commands.spawn_bundle(TextModeBundle::new(&tiles, &mut materials, 1, x, y, Color::BEIGE, Color::TOMATO, meshes.tile.clone()));
        }
    }
}

fn update_color(
    mut materials: ResMut<Assets<TileMaterial>>,
    all_tiles: Res<Tiles>,
    tiles: Query<(&TilePos, &Handle<TileMaterial>)>,
) {
    // let x: u8 = random::<u8>() % 8;
    // let y: u8 = random::<u8>() % 8;
    // for (pos, handle) in tiles.iter() {
    //     if pos.x == x as i32 && pos.y == y as i32 {
    //         let mut tileMaterial = materials.get_mut(handle).unwrap();
    //         tileMaterial.fg = Color::MIDNIGHT_BLUE;
    //         tileMaterial.texture = all_tiles.tiles.get(&2).unwrap().clone();
    //     }
    // }
}

#[derive(Debug, Clone, Component, TypeUuid)]
#[uuid = "eb3bfce5-5e0d-4a0e-bf7c-dec3e8a6d330"]
pub struct TileMaterial {
    texture: Handle<Image>,
    bg: Color,
    fg: Color,
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
use std::collections::HashMap;
use bevy::app::Plugin;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::sprite::Mesh2dHandle;
use image::GenericImageView;
use crate::App;
use crate::tile_material::TileMaterial;

pub struct TextModePlugin;

impl Plugin for TextModePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

#[derive(Component)]
pub struct BasicMesh {
    pub(crate) tile: Handle<Mesh>
}

#[derive(Component)]
pub struct Tiles {
    tiles: HashMap<usize, Handle<Image>>
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
    pub(crate) fn new(tiles: &Res<Tiles>, materials: &mut ResMut<Assets<TileMaterial>>, index: usize, x: i32, y: i32, bg: Color, fg: Color, mesh: Handle<Mesh>) -> Self {
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
    images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    init_spritesheet("assets/MRMOTEXT.png", 8, images, &mut commands);

    commands.insert_resource(BasicMesh {
        tile: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(8.0, 8.0))))
    });
}

fn init_spritesheet(
    path: &str,
    size: u32,
    mut images: ResMut<Assets<Image>>,
    commands: &mut Commands,
) {
    let img = image::open(path).expect("File not found");
    let tile_width = img.width() / size;
    let tile_height = img.height() / size;
    let mut tiles_vec = vec![];
    for _i in 0..(tile_width * tile_height) {
        tiles_vec.push(vec![]);
    }
    for pixel in img.pixels() {
        let x = pixel.0;
        let y = pixel.1;
        let n = x / size + y / size * tile_width;
        if let Some(tile) = tiles_vec.get_mut(n as usize) {
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

    for i in 0..tiles_vec.len() {
        let handle = images.add(Image::new(
            Extent3d {
                width: size,
                height: size,
                depth_or_array_layers: 1
            },
            TextureDimension::D2,
            tiles_vec.get(i).unwrap().clone(),
            TextureFormat::Rgba8UnormSrgb
        ));
        tiles_hm.insert(i, handle);
    }

    commands.insert_resource(Tiles { tiles: tiles_hm });
}
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

#[derive(Component, Copy, Clone, Eq, PartialEq, Hash)]
pub struct TileId {
    pub(crate) index: usize,
    pub(crate) flip: bool,
    pub(crate) rotation: u8,
}

impl TileId {
    pub fn new() -> Self {
        TileId {
            index: 0,
            flip: false,
            rotation: 0
        }
    }

    pub fn rotate(&mut self) {
        self.rotation = (self.rotation + 1) % 4;
    }

    pub fn flip(&mut self) {
        self.flip = !self.flip;
    }
}

#[derive(Component)]
pub struct Tiles {
    tiles: HashMap<TileId, Handle<Image>>
}

#[derive(Component, Copy, Clone, Eq, PartialEq)]
pub struct TilePos {
    pub x: i32,
    pub y: i32,
}

#[derive(Bundle, Clone)]
pub struct TextModeBundle {
    pub pos: TilePos,
    pub id: TileId,
    pub mesh: Mesh2dHandle,
    pub material: Handle<TileMaterial>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}

impl TextModeBundle {
    pub(crate) fn new(tiles: &Res<Tiles>, materials: &mut ResMut<Assets<TileMaterial>>, id: &TileId, x: i32, y: i32, bg: Color, fg: Color, mesh: Handle<Mesh>) -> Self {
        let texture = tiles.tiles.get(id).expect("Couldn't find tile.");
        TextModeBundle {
            pos: TilePos { x, y },
            id: id.clone(),
            mesh: mesh.into(),
            material: materials.add(TileMaterial { texture: texture.clone(), bg, fg }),
            transform: Transform {
                translation: Vec3::new(x as f32 * 8.0 + 26.0, y as f32 * 8.0, 0.0),
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
    let mut tiles_vec: Vec<Vec<(u8, u8, u8, u8)>> = vec![];
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
                    tile.push((0, 0, 0, 255));
                }
                _ => {
                    tile.push((255, 255, 255, 255));
                }
            }
        }
    }

    let mut tiles_hm = HashMap::new();
    let extent3d = Extent3d {
        width: size,
        height: size,
        depth_or_array_layers: 1
    };

    let get_handle = |id: &TileId, input: &Vec<(u8, u8, u8, u8)>| Image::new(
        extent3d.clone(),
        TextureDimension::D2,
        flip(id, size, input.clone()),
        TextureFormat::Rgba8UnormSrgb
    );

    for index in 0..tiles_vec.len() {
        for flip in 0..2 {
            for rotation in 0..4 {
                let id = TileId { index, flip: flip == 1, rotation };
                let handle = images.add(get_handle(&id, tiles_vec.get(index).unwrap()));
                tiles_hm.insert(id, handle);
            }
        }
    }

    commands.insert_resource(Tiles { tiles: tiles_hm });
}

fn flip(id: &TileId, size: u32, input: Vec<(u8, u8, u8, u8)>) -> Vec<u8> {
    let mut result = vec![];
    let mut step = vec![];
    let mut input = input.clone();

    while input.len() > 0 {
        let x = input.remove(0);
        step.push(x);
        if step.len() == size as usize {
            result.push(step.clone());
            step.clear();
        }
    }

    if id.flip { result.iter_mut().for_each(|v| v.reverse()) }

    for _i in 0..(id.rotation % 4) {
        let before = result.clone();
        result.iter_mut().for_each(|v| v.clear());
        before.iter().for_each(|v| for j in 0..v.len() {
            result.get_mut(j).unwrap().push(v.get(size as usize - j - 1).unwrap().clone());
        })
    }

    return result.iter()
        .flat_map(|x: &Vec<(u8, u8, u8, u8)>|
            x.iter()
                .flat_map(|tup: &(u8, u8, u8, u8)|
                    [tup.0, tup.1, tup.2, tup.3].into_iter().clone()
                )
        ).collect::<Vec<u8>>();
}
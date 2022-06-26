use bevy::math::vec2;
use bevy::prelude::*;
use bevy::sprite::Material2dPlugin;
use bevy::window::PresentMode;
use rand::random;
use crate::colors::{ColorPlugin, Colors};
use crate::cursor::CursorPlugin;
use crate::tile_material::TileMaterial;
use crate::tiles::{BasicMesh, TextModeBundle, TextModePlugin, TileId, TilePos, Tiles};
use crate::gui::GuiPlugin;

mod tiles;
mod tile_material;
mod colors;
mod gui;
mod cursor;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(Material2dPlugin::<TileMaterial>::default())
        .add_plugin(TextModePlugin)
        .add_plugin(ColorPlugin)
        .add_plugin(GuiPlugin)
        .add_plugin(CursorPlugin)
        .insert_resource(WindowDescriptor {
            title: "bevy_textmode".to_string(),
            present_mode: PresentMode::Immediate,
            ..Default::default()
        })
        .insert_resource(Canvas {
            tile_size: 8,
            width: 32,
            height: 18,
            offset: vec2(26.0, 0.0),
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
struct MainCamera;

#[derive(Debug, Clone)]
struct Canvas {
    tile_size: u32,
    width: u32,
    height: u32,
    offset: Vec2,
}

fn setup(
    mut commands: Commands,
    canvas: Res<Canvas>,
) {
    let mut camera = OrthographicCameraBundle::new_2d();
    let tile = canvas.tile_size as f32;
    camera.transform = Transform {
        translation: Vec3::new((canvas.width - 1) as f32 / 2. * tile, (canvas.height - 1) as f32 / 2. * tile, 999.0),
        scale: Vec3::new(0.25, 0.25, 1.0),
        ..Default::default()
    };
    commands
        .spawn_bundle(camera)
        .insert(MainCamera);
}

fn spawn_grid(
    mut commands: Commands,
    mut materials: ResMut<Assets<TileMaterial>>,
    tiles: Res<Tiles>,
    canvas: Res<Canvas>,
    colors: Res<Colors>,
    meshes: Res<BasicMesh>,
) {
    for x in 0..canvas.width {
        for y in 0..canvas.height {
            let i: usize = random::<usize>() % 1024;
            let flip = random::<bool>();
            let rotation = random::<u8>() % 4;
            let bg: usize = random::<usize>() % 16;
            let fg: usize = random::<usize>() % 16;
            commands.spawn_bundle(TextModeBundle::new(
                &tiles, &mut materials,
                &TileId { index: i, flip, rotation},
                x, y,
                colors.get(bg), colors.get(fg),
                meshes.tile.clone(), canvas.as_ref()
            ));
        }
    }
}

fn update_color(
    materials: ResMut<Assets<TileMaterial>>,
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
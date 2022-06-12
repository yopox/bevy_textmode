use bevy::prelude::*;
use bevy::sprite::Material2dPlugin;
use bevy::window::PresentMode;
use rand::random;
use crate::colors::{ColorPlugin, Colors};
use crate::tile_material::TileMaterial;
use crate::tiles::{BasicMesh, TextModeBundle, TextModePlugin, TilePos, Tiles};

mod tiles;
mod tile_material;
mod colors;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(Material2dPlugin::<TileMaterial>::default())
        .add_plugin(TextModePlugin)
        .add_plugin(ColorPlugin)
        .insert_resource(WindowDescriptor {
            title: "bevy_textmode".to_string(),
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

fn setup(
    mut commands: Commands,
) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform = Transform {
        translation: Vec3::new(15.5 * 8.0, 8.5 * 8.0, 0.0),
        scale: Vec3::new(0.25, 0.25, 1.0),
        ..Default::default()
    };
    commands.spawn_bundle(camera);
}

fn spawn_grid(
    mut commands: Commands,
    mut materials: ResMut<Assets<TileMaterial>>,
    tiles: Res<Tiles>,
    colors: Res<Colors>,
    meshes: Res<BasicMesh>,
) {
    for x in 0..32 {
        for y in 0..18 {
            let i: usize = random::<usize>() % 1024;
            let bg: usize = random::<usize>() % 16;
            let fg: usize = random::<usize>() % 16;
            commands.spawn_bundle(TextModeBundle::new(&tiles, &mut materials, i, x, y, colors.get(bg), colors.get(fg), meshes.tile.clone()));
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
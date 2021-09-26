use bevy::prelude::*;
use std::ops::Deref;
use rand::prelude::*;

const WIDTH: u16 = 10;
const HEIGHT: u16 = 6;
const SCALE: f32 = 8.;
const SIZE: f32 = 8.;
const SCREENW: f32 = 1280.;
const SCREENH: f32 = 720.;

struct Materials {
    tileset: Handle<TextureAtlas>,
}
struct Tile;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(WindowDescriptor {
            title: "rtemo".to_string(),
            width: SCREENW,
            height: SCREENH,
            vsync: true,
            ..Default::default()
        })
        .add_startup_system(setup.system())
        .add_system(shuffle.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Res<Windows>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // CAMERA
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform = Transform {
        translation: Vec3::new(SCALE * SIZE * WIDTH as f32 / 2., SCALE * SIZE * HEIGHT as f32 / 2., 0.),
        ..Default::default()
    };
    commands.spawn_bundle(camera);

    let mrmo_handle = asset_server.load("MRMOTEXT.png");
    let texture_atlas = TextureAtlas::from_grid(mrmo_handle, Vec2::new(8.0, 8.0), 32, 32);
    let texture_handle = texture_atlases.add(texture_atlas);
    let left: f32 = SCALE * SIZE / 2.;
    let bottom: f32 = SCALE * SIZE / 2.;
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            commands
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: texture_handle.clone(),
                    transform: Transform {
                        translation: Vec3::new(&left + x as f32 * SCALE * SIZE, &bottom + y as f32 * SCALE * SIZE, 0.),
                        scale: Vec3::new(SCALE, SCALE, 1.),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Tile);
        }
    }
}

fn shuffle(
    keys: Res<Input<KeyCode>>,
    mut tiles: Query<(&Tile, &mut TextureAtlasSprite)>,
) {
    let mut rng = rand::thread_rng();
    if keys.just_pressed(KeyCode::R) {
        for (_, mut sprite) in tiles.iter_mut() {
            sprite.index = (rng.gen::<f64>() * 512.0) as u32;
            if rng.gen::<f64>() < 0.3 { sprite.index = 0 }
        }
    }
}
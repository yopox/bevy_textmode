use bevy::prelude::*;
use std::ops::Deref;
use rand::prelude::*;

const WIDTH: u16 = 10;
const HEIGHT: u16 = 6;
const SCALE: f32 = 8.;
const SIZE: f32 = 8.;
const TILE_SIZE: f32 = SIZE * SCALE;
const SCREENW: f32 = 1280.;
const SCREENH: f32 = 720.;

pub struct Materials {
    tileset: Handle<TextureAtlas>,
    cursor: Handle<ColorMaterial>,
}
struct MainCamera;
struct Tile;
struct Cursor;

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
        .add_startup_stage(
            "game_setup_actors",
            SystemStage::single(spawn_grid.system()),
        )
        .add_system(shuffle.system())
        .add_system(update_cursor.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Ressources
    let texture_atlas = TextureAtlas::from_grid(asset_server.load("MRMOTEXT.png"), Vec2::new(8.0, 8.0), 32, 32);
    commands.insert_resource(Materials {
        tileset: texture_atlases.add(texture_atlas),
        cursor: materials.add(asset_server.load("cursor.png").into()),
    });

    // Camera
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform = Transform {
        translation: Vec3::new(TILE_SIZE * WIDTH as f32 / 2., TILE_SIZE * HEIGHT as f32 / 2., 999.),
        ..Default::default()
    };
    commands
        .spawn_bundle(camera)
        .insert(MainCamera);
}

fn spawn_grid(
    mut commands: Commands,
    materials: Res<Materials>,
) {
    let left: f32 = TILE_SIZE / 2.;
    let bottom: f32 = TILE_SIZE / 2.;
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            commands
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: materials.tileset.clone(),
                    transform: Transform {
                        translation: Vec3::new(&left + x as f32 * TILE_SIZE, &bottom + y as f32 * TILE_SIZE, 0.),
                        scale: Vec3::new(SCALE, SCALE, 1.),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Tile);
        }
    }

    // Cursor
    commands.spawn_bundle(SpriteBundle {
        material: materials.cursor.clone(),
        visible: Visible {
            is_visible: false,
            is_transparent: true,
        },
        transform: Transform {
            translation: Vec3::new(0., 0., 1.),
            ..Default::default()
        },
        ..Default::default()
    }).insert(Cursor);
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

fn update_cursor(
    windows: Res<Windows>,
    mut q: QuerySet<(
        Query<&Transform, With<MainCamera>>,
        Query<(&mut Transform, &mut Visible), With<Cursor>>
    )>,
) {
    let wnd = windows.get_primary().unwrap();
    if let Some(pos) = wnd.cursor_position() {
        let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
        let p = pos - size / 2.0;
        let camera_transform = q.q0_mut().single().unwrap();
        let pos_wld = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);
        let (mut cursor_transform, mut visible) = q.q1_mut().single_mut().expect("Cursor not found");
        if pos_wld.x < 0. || pos_wld.x > WIDTH as f32 * TILE_SIZE || pos_wld.y < 0. || pos_wld.y > HEIGHT as f32 * TILE_SIZE  {
            visible.is_visible = false;
        } else {
            visible.is_visible = true;
            cursor_transform.translation.x = TILE_SIZE * (0.5 + (pos_wld.x as u16 / TILE_SIZE as u16) as f32);
            cursor_transform.translation.y = TILE_SIZE * (0.5 + (pos_wld.y as u16 / TILE_SIZE as u16) as f32);
        }
    }
}
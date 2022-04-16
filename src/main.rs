use bevy::prelude::*;
use bevy::window::PresentMode;
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
    cursor: Handle<Image>,
}

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct Tile {
    x: u16,
    y: u16,
    bg: Color,
    fg: Color,
}

impl Tile {
    fn new(x: u16, y: u16) -> Self {
        Tile {
            x,
            y,
            bg: Color::WHITE,
            fg: Color::BLACK,
        }
    }
}

#[derive(Component)]
struct RecomputeTexture;

#[derive(Component)]
struct Cursor {
    x: u16,
    y: u16,
}

#[derive(Component)]
struct Selected;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WindowDescriptor {
            title: "rtemo".to_string(),
            width: SCREENW,
            height: SCREENH,
            present_mode: PresentMode::Immediate,
            ..Default::default()
        })
        .add_startup_system(setup)
        .add_startup_stage(
            "game_setup_actors",
            SystemStage::single(spawn_grid),
        )
        .add_system(shuffle)
        .add_system(update_cursor.label("cursor"))
        .add_system(flip.after("cursor"))
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Ressources
    let texture_atlas = TextureAtlas::from_grid(asset_server.load("MRMOTEXT.png"), Vec2::new(8.0, 8.0), 32, 32);
    commands.insert_resource(Materials {
        tileset: texture_atlases.add(texture_atlas),
        cursor: asset_server.load("cursor.png"),
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
                .insert(Tile::new(x, y));
        }
    }

    // Cursor
    commands.spawn_bundle(SpriteBundle {
        texture: materials.cursor.clone(),
        visibility: Visibility {
            is_visible: false,
        },
        transform: Transform {
            translation: Vec3::new(0., 0., 1.),
            ..Default::default()
        },
        ..Default::default()
    }).insert(Cursor { x: 0, y: 0 });
}

fn shuffle(
    keys: Res<Input<KeyCode>>,
    mut tiles: Query<&mut TextureAtlasSprite, With<Tile>>,
) {
    let mut rng = rand::thread_rng();
    if keys.just_pressed(KeyCode::R) {
        for mut sprite in tiles.iter_mut() {
            sprite.index = (rng.gen::<f64>() * 512.0) as usize;
            if rng.gen::<f64>() < 0.3 { sprite.index = 0 }
        }
    }
}

fn update_cursor(
    windows: Res<Windows>,
    mut q: ParamSet<(
        Query<&Transform, With<MainCamera>>,
        Query<(&mut Transform, &mut Visibility, &mut Cursor)>,
    )>,
) {
    let wnd = windows.get_primary().unwrap();
    if let Some(pos) = wnd.cursor_position() {
        let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
        let p = pos - size / 2.0;
        let pos_wld = q.p0().single().compute_matrix() * p.extend(0.0).extend(1.0);
        let mut query = q.p1();
        let (mut cursor_pos, mut visible, mut cursor) = query.single_mut();
        visible.is_visible = !(pos_wld.x < 0.
            || pos_wld.x > WIDTH as f32 * TILE_SIZE
            || pos_wld.y < 0.
            || pos_wld.y > HEIGHT as f32 * TILE_SIZE);
        if visible.is_visible {
            let x = pos_wld.x as u16 / TILE_SIZE as u16;
            let y = pos_wld.y as u16 / TILE_SIZE as u16;
            cursor_pos.translation.x = TILE_SIZE * (0.5 + x as f32);
            cursor_pos.translation.y = TILE_SIZE * (0.5 + y as f32);
            cursor.x = x;
            cursor.y = y;
        }
    }
}

fn flip(
    keys: Res<Input<KeyCode>>,
    mut q: ParamSet<(
        Query<(&Cursor, &Visibility)>,
        Query<(&mut TextureAtlasSprite, &Tile)>,
    )>,
) {
    if keys.just_pressed(KeyCode::LAlt) || keys.just_pressed(KeyCode::LControl) {
        let (x, y) = {
            let p = q.p0();
            let (cursor, visible) = p.single();
            if !visible.is_visible { return; }
            (cursor.x, cursor.y)
        };
        let mut p =  q.p1();
        for (mut sprite, tile) in p.iter_mut() {
            if tile.x == x && tile.y == y {
                if keys.just_pressed(KeyCode::LAlt) { sprite.flip_x = !sprite.flip_x; }
                if keys.just_pressed(KeyCode::LControl) { sprite.flip_y = !sprite.flip_y; }
                break;
            }
        }
    }
}
use bevy::prelude::*;
use crate::{BasicMesh, Canvas, Colors, MainCamera, TextModeBundle, TileId, TileMaterial, Tiles};
use crate::gui::UiState;

pub(crate) struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_stage(
                "game_setup_cursor",
                SystemStage::single(setup),
            )
            .add_system(update_tile)
            .add_system(update_cursor);
    }
}

#[derive(Component)]
struct Cursor;

#[derive(Component)]
struct TileCursor;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<TileMaterial>>,
    tiles: Res<Tiles>,
    canvas: Res<Canvas>,
    colors: Res<Colors>,
    meshes: Res<BasicMesh>,
) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Default::default(),
            texture: asset_server.load("cursor.png"),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 10.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Cursor);

    commands
        .spawn_bundle(TextModeBundle::new(
            &tiles, &mut materials,
            &TileId { index: 0, flip: false, rotation: 0 },
            0, 0,
            colors.black, colors.red_4,
            meshes.tile.clone(), canvas.as_ref()
        ))
        .insert(TileCursor);
}

fn update_tile(
    mut materials: ResMut<Assets<TileMaterial>>,
    all_tiles: Res<Tiles>,
    colors: Res<Colors>,
    ui_state: Res<UiState>,
    mut q: Query<(&mut TileId, &Handle<TileMaterial>), (With<TileCursor>)>,
) {
    let (_, handle) = q.single_mut();
    let mut tile_material = materials.get_mut(handle).unwrap();
    tile_material.bg = colors.get(ui_state.bg);
    tile_material.fg = colors.get(ui_state.fg);
    tile_material.texture = all_tiles.tiles.get(&ui_state.tile_id).unwrap().clone();
}

fn update_cursor(
    windows: Res<Windows>,
    canvas: Res<Canvas>,
    mut q: ParamSet<(
        Query<&Transform, With<MainCamera>>,
        Query<(&mut Transform, &mut Visibility), With<Cursor>>,
        Query<(&mut Transform, &mut Visibility), With<TileCursor>>,
    )>,
) {
    let wnd = windows.get_primary().unwrap();
    if let Some(pos) = wnd.cursor_position() {
        let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
        let p = pos - size / 2.0;
        let pos_wld = q.p0().single().compute_matrix() * p.extend(0.0).extend(1.0);

        let tile = canvas.tile_size as f32;
        let (x, y) = (pos_wld.x + tile / 2., pos_wld.y + tile / 2.);
        let display =  x >= canvas.offset.x
            && x < canvas.width as f32 * tile + canvas.offset.x
            && y >= canvas.offset.y
            && y < canvas.height as f32 * tile + canvas.offset.y;

        let x = ((pos_wld.x + tile / 2. - canvas.offset.x) / tile) as i32;
        let y = ((pos_wld.y + tile / 2. - canvas.offset.y) / tile) as i32;

        let mut query = q.p1();
        let (_, mut visibility) = query.single_mut();
        visibility.is_visible = false;
        // if display {
        //     cursor_pos.translation.x = x as f32 * tile + canvas.offset.x;
        //     cursor_pos.translation.y = y as f32 * tile + canvas.offset.y;
        //     visibility.is_visible = true;
        // } else {
        //     visibility.is_visible = false;
        // }

        let mut query = q.p2();
        let (mut tile_pos, mut visibility) = query.single_mut();
        if display {
            tile_pos.translation.x = x as f32 * tile + canvas.offset.x;
            tile_pos.translation.y = y as f32 * tile + canvas.offset.y;
            visibility.is_visible = true;
        } else {
            visibility.is_visible = false;
        }
    }
}
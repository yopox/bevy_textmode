use bevy::ecs::schedule::IntoRunCriteria;
use bevy::math::Vec4Swizzles;
use bevy::prelude::*;
use crate::{Canvas, MainCamera};
use crate::KeyCode::P;

pub(crate) struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup)
            .add_system(update_cursor);
    }
}

#[derive(Component)]
struct Cursor;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
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
}

fn update_cursor(
    windows: Res<Windows>,
    canvas: Res<Canvas>,
    mut q: ParamSet<(
        Query<&Transform, With<MainCamera>>,
        Query<(&mut Transform, &mut Visibility), (With<Cursor>)>,
    )>,
) {
    let wnd = windows.get_primary().unwrap();
    if let Some(pos) = wnd.cursor_position() {
        let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
        let p = pos - size / 2.0;
        let pos_wld = q.p0().single().compute_matrix() * p.extend(0.0).extend(1.0);
        let mut query = q.p1();
        let (mut cursor_pos, mut visibility) = query.single_mut();

        let tile = canvas.tile_size as f32;
        let (x, y) = (pos_wld.x + tile / 2., pos_wld.y + tile / 2.);

        if x >= canvas.offset.x && x < canvas.width as f32 * tile + canvas.offset.x
            && y >= canvas.offset.y && y < canvas.height as f32 * tile + canvas.offset.y {
            let x = ((pos_wld.x + tile / 2. - canvas.offset.x) / tile) as i32;
            let y = ((pos_wld.y + tile / 2. - canvas.offset.y) / tile) as i32;

            cursor_pos.translation.x = x as f32 * tile + canvas.offset.x;
            cursor_pos.translation.y = y as f32 * tile + canvas.offset.y;
            visibility.is_visible = true;
        } else {
            visibility.is_visible = false;
        }

    }
}
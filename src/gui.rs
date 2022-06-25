use bevy::ecs::system::Resource;
use bevy::math::vec2;
use bevy::prelude::*;
use bevy::render::render_resource::TextureId;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_egui::egui::color_picker::show_color;
use bevy_egui::egui::{emath, FontData, FontDefinitions, FontFamily, ImageData, pos2, Pos2, Rgba, TextEdit, Ui, Widget};
use egui_extras::RetainedImage;
use crate::{Colors, TileId};

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(EguiPlugin)
            .init_resource::<UiState>()
            .add_startup_system(setup)
            .add_system(update_uv)
            .add_system(ui);
    }
}

struct UiState {
    image: Option<RetainedImage>,
    tile: Option<egui::Image>,
    tile_id: TileId,
    selected_rect: egui::Rect,
}

impl Default for UiState {
    fn default() -> Self {
        UiState {
            image: None,
            tile: None,
            tile_id: TileId::new(),
            selected_rect: egui::Rect { min: pos2(0., 0.), max: pos2(7., 7.) },
        }
    }
}

fn setup(
    mut egui_ctx: ResMut<EguiContext>,
    mut ui_state: ResMut<UiState>,
) {
    let image = image::io::Reader::open("assets/MRMOTEXT.png").unwrap().decode().unwrap();
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    ui_state.image = Some(
        RetainedImage::from_color_image("tileset", egui::ColorImage::from_rgba_unmultiplied(
            size,
            pixels.as_slice(),
        ))
    );
    ui_state.tile = Some(egui::Image::new(
        ui_state.image.as_ref().unwrap().texture_id(egui_ctx.ctx_mut()),
        egui::vec2(32., 32.)
    ));

    let mut fonts = FontDefinitions::default();
    fonts.font_data
        .insert("JB Mono".to_owned(),
                FontData::from_static(include_bytes!("../assets/JetBrainsMono-Regular.ttf"))
        );
    vec![&FontFamily::Proportional, &FontFamily::Monospace].iter().for_each(|key| {
        fonts.families
            .get_mut(key).unwrap()
            .insert(0, "JB Mono".to_owned());
    });

    egui_ctx.ctx_mut().set_fonts(fonts);
}

fn update_uv(
    mut ui_state: ResMut<UiState>,
) {
}

fn to_rgba(color: Color) -> Rgba {
    Rgba::from_rgb(color.r(), color.g(), color.b())
}

fn ui(
    mut egui_ctx: ResMut<EguiContext>,
    colors: Res<Colors>,
    mut ui_state: ResMut<UiState>,
) {
    let mut remove = false;

    egui::SidePanel::left("side_panel")
        .default_width(200.0)
        .resizable(false)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.add_space(16.);

            ui.horizontal(|ui| {
                ui.centered_and_justified(|ui| ui.heading("IMPERATRICES"));
            });

            ui.add_space(8.);

            let color_ui = |ui: &mut Ui, color: &Color| {
                ui.horizontal(|ui| {
                    ui.add_space(16.);
                    show_color(ui, to_rgba(*color), egui::Vec2::new(64.0, 20.0));
                    ui.add_space(8.);
                    let _ = ui.button("-BG-");
                    ui.add_space(4.);
                    let _ = ui.button("-FG-");
                });
                ui.add_space(2.);
            };

            vec![colors.black,
                 colors.blue_0, colors.blue_1, colors.blue_2, colors.blue_3, colors.blue_4,
                 colors.red_0, colors.red_1, colors.red_2, colors.red_3, colors.red_4,
                 colors.yellow_0, colors.yellow_1, colors.yellow_2, colors.yellow_3, colors.yellow_4]
                .iter()
                .for_each(|c| { color_ui(ui, c); ui.add_space(4.); });

            ui.add_space(16.);

            if ui_state.tile.is_some() {
                ui.horizontal(|ui| {
                    ui.centered_and_justified(|ui| ui.heading("Selected tile"));
                });

                ui.add_space(8.);

                ui.horizontal(|ui| {
                    ui.add_space(24.);
                    let y = ui_state.tile_id.index / 32;
                    let x = ui_state.tile_id.index % 32;
                    ui.add(ui_state.tile.unwrap().uv(egui::Rect::from_min_max(
                        Pos2::new(x as f32 / 32., y as f32 / 32.),
                        Pos2::new((x + 1) as f32 / 32., (y + 1) as f32 / 32.)
                    )));
                    ui.add_space(4.);
                    if ui.button("ROTATE").clicked() { ui_state.tile_id.rotate(); }
                    ui.add_space(4.);
                    if ui.button("FLIP").clicked() { ui_state.tile_id.flip(); }
                });

                ui.add_space(8.);

                ui.horizontal(|ui| {
                    ui.add_space(24.);
                    ui.heading("#");
                    let mut goto = ui_state.tile_id.index as f32;
                    ui.add(egui::DragValue::new::<f32>(&mut goto).speed(1.0));
                    ui_state.tile_id.index = goto as usize;

                    ui.add_space(17.);

                    let mut goto = ui_state.tile_id.index.to_string();
                    ui.add(TextEdit::singleline(&mut goto).desired_width(49.));

                    if let Ok(n) = usize::from_str_radix(goto.as_str(), 10) {
                        ui_state.tile_id.index = n
                    }
                });

                ui.add_space(8.);

                ui.horizontal(|ui| {
                    ui.add_space(24.);
                    ui.heading("X");
                    let mut goto_x = (ui_state.tile_id.index % 32) as f32;
                    ui.add(egui::DragValue::new::<f32>(&mut goto_x).speed(1.0));

                    ui.add_space(17.);

                    ui.heading("Y");
                    let mut goto_y = (ui_state.tile_id.index / 32) as f32;
                    ui.add(egui::DragValue::new::<f32>(&mut goto_y).speed(1.0));

                    ui_state.tile_id.index = goto_x as usize + goto_y as usize * 32;
                });

                ui.add_space(8.);

                ui.horizontal(|ui| {
                    ui.add_space(24.);
                    if ui.button("- 1").clicked() { ui_state.tile_id.index -= 1; }
                    ui.add_space(2.);
                    if ui.button("-10").clicked() { ui_state.tile_id.index -= 10; }
                    ui.add_space(2.);
                    if ui.button("+ 1").clicked() { ui_state.tile_id.index += 1; }
                    ui.add_space(2.);
                    if ui.button("+10").clicked() { ui_state.tile_id.index += 10; }
                });
            }
        });
}
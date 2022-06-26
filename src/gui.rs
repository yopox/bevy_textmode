use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_egui::egui::color_picker::show_color;
use bevy_egui::egui::{Color32, FontData, FontDefinitions, FontFamily, Pos2, TextEdit};
use egui_extras::RetainedImage;
use crate::{Canvas, Colors, TileId};

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(EguiPlugin)
            .init_resource::<UiState>()
            .add_startup_system(setup)
            .add_system(ui);
    }
}

pub struct UiState {
    image: Option<RetainedImage>,
    tile: Option<egui::Image>,
    pub tile_id: TileId,
    pub fg: usize,
    pub bg: usize,
}

impl Default for UiState {
    fn default() -> Self {
        UiState {
            image: None,
            tile: None,
            tile_id: TileId::new(),
            fg: 10,
            bg: 0,
        }
    }
}

fn setup(
    mut egui_ctx: ResMut<EguiContext>,
    mut ui_state: ResMut<UiState>,
    canvas: Res<Canvas>,
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
        egui::vec2((canvas.tile_size * 4) as f32, (canvas.tile_size * 4) as f32)
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

fn ui(
    mut egui_ctx: ResMut<EguiContext>,
    colors: Res<Colors>,
    mut ui_state: ResMut<UiState>,
) {
    egui::SidePanel::left("side_panel")
        .default_width(200.0)
        .resizable(false)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.add_space(16.);

            ui.horizontal(|ui| {
                ui.centered_and_justified(|ui| ui.heading("IMPERATRICES"));
            });

            ui.add_space(8.);

            for i in 0..15 {
                ui.horizontal(|ui| {
                    ui.add_space(16.);
                    let c1 = colors.get(i);
                    let c2 = Color32::from_rgb((c1.r() * 255.) as u8, (c1.g() * 255.) as u8, (c1.b() * 255.) as u8);
                    show_color(ui, c2, egui::Vec2::new(64.0, 20.0));
                    ui.add_space(8.);
                    if ui.button("-BG-").clicked() { ui_state.bg = i; }
                    ui.add_space(4.);
                    if ui.button("-FG-").clicked() { ui_state.fg = i; }
                });
                ui.add_space(2.);
            }

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
                    if ui.button("FLIP").clicked() { ui_state.tile_id.flip(); ui_state.tile_id.rotate(); ui_state.tile_id.rotate(); }
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
                    ui.add(egui::DragValue::new::<f32>(&mut goto_x).speed(0.2));

                    ui.add_space(17.);

                    ui.heading("Y");
                    let mut goto_y = (ui_state.tile_id.index / 32) as f32;
                    ui.add(egui::DragValue::new::<f32>(&mut goto_y).speed(0.2));

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
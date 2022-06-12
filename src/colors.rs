use bevy::prelude::{Color, Commands, Component};
use crate::{App, Plugin};

#[derive(Component)]
pub struct Colors {
    pub black: Color,
    pub blue_0: Color,
    pub blue_1: Color,
    pub blue_2: Color,
    pub blue_3: Color,
    pub blue_4: Color,
    pub red_0: Color,
    pub red_1: Color,
    pub red_2: Color,
    pub red_3: Color,
    pub red_4: Color,
    pub yellow_0: Color,
    pub yellow_1: Color,
    pub yellow_2: Color,
    pub yellow_3: Color,
    pub yellow_4: Color,
}

impl Colors {
    pub(crate) fn get(&self, i: usize) -> Color {
        match i % 15 {
            1 => self.blue_0,
            2 => self.blue_1,
            3 => self.blue_2,
            4 => self.blue_3,
            5 => self.blue_4,
            6 => self.red_0,
            7 => self.red_1,
            8 => self.red_2,
            9 => self.red_3,
            10 => self.red_4,
            11 => self.yellow_0,
            12 => self.yellow_1,
            13 => self.yellow_2,
            14 => self.yellow_3,
            15 => self.yellow_4,
            _ => self.black,
        }
    }
}

pub struct ColorPlugin;

impl Plugin for ColorPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(
    mut commands: Commands
) {
    commands.insert_resource(Colors {
        black: Color::hex("010103").unwrap(),
        blue_0: Color::hex("161a2d").unwrap(),
        blue_1: Color::hex("1f3846").unwrap(),
        blue_2: Color::hex("323f74").unwrap(),
        blue_3: Color::hex("20b7af").unwrap(),
        blue_4: Color::hex("6dc8d7").unwrap(),
        red_0: Color::hex("871b12").unwrap(),
        red_1: Color::hex("e94b50").unwrap(),
        red_2: Color::hex("f19cd1").unwrap(),
        red_3: Color::hex("ffc9f7").unwrap(),
        red_4: Color::hex("ffe9fc").unwrap(),
        yellow_0: Color::hex("774b1a").unwrap(),
        yellow_1: Color::hex("e18a1a").unwrap(),
        yellow_2: Color::hex("f6d32d").unwrap(),
        yellow_3: Color::hex("e5ca9f").unwrap(),
        yellow_4: Color::hex("eee4ca").unwrap(),
    });
}
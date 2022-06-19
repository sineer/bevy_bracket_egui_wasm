// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::{App, ClearColor, Color, Msaa};
use bevy::DefaultPlugins;
use bevy_game::GamePlugin;

use bracket_bevy::prelude::*;

mod a_star_mouse;

use crate::a_star_mouse::{setup, tick};

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        //        .insert_resource(WindowDescriptor {
        //            width: 800.,
        //            height: 600.,
        //            title: "Bevy game".to_string(), // ToDo
        //            ..Default::default()
        //        })
        .add_plugin(BTermBuilder::simple_80x50().with_random_number_generator(true))
        .add_plugins(DefaultPlugins)
        //        .add_plugin(GamePlugin)
        .add_startup_system(setup)
        .add_system(tick)
        .run();
}

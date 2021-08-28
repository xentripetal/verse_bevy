// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::ecs::prelude::ResMut;
use bevy::prelude::{App, ClearColor, Color, Msaa, WindowDescriptor};
use bevy::window::WindowMode::BorderlessFullscreen;
use bevy::DefaultPlugins;
use bevy_egui::{egui, EguiContext, EguiPlugin};
#[cfg(target_arch = "wasm32")]
use bevy_webgl2;

use bevy_ecs_tilemap::TilemapPlugin;
use verse::GamePlugin;

fn main() {
    let mut app = App::build();
    app
        //.insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .insert_resource(WindowDescriptor {
            //mode: BorderlessFullscreen,
            title: "Project Verse".to_string(), // ToDo
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(GamePlugin);

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    app.run();
}

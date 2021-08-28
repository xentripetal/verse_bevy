use bevy::prelude::*;
use bevy_egui::egui::Pos2;
use bevy_egui::{egui, EguiContext, EguiSettings};

use crate::GameState;

pub struct MainMenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(GameState::Menu)
                .with_system(update_ui_scale_factor.system())
                .with_system(ui_example.system()),
        );
    }
}

pub fn update_ui_scale_factor(
    keyboard_input: Res<Input<KeyCode>>,
    mut toggle_scale_factor: Local<Option<bool>>,
    mut egui_settings: ResMut<EguiSettings>,
    windows: Res<Windows>,
) {
    if keyboard_input.just_pressed(KeyCode::Slash) || toggle_scale_factor.is_none() {
        *toggle_scale_factor = Some(!toggle_scale_factor.unwrap_or(true));

        if let Some(window) = windows.get_primary() {
            let scale_factor = if toggle_scale_factor.unwrap() {
                1.0
            } else {
                1.0 / window.scale_factor()
            };
            egui_settings.scale_factor = scale_factor;
        }
    }
}

fn ui_example(egui_context: ResMut<EguiContext>, mut state: ResMut<State<GameState>>) {
    egui::Area::new("canvas")
        .default_pos(Pos2::new(0., 0.))
        .movable(false)
        .show(egui_context.ctx(), |ui| {
            if ui.button("Play Game").clicked() {
                state.set(GameState::Playing).unwrap();
            }
            if ui.button("Play Game").clicked() {
                state.set(GameState::Playing).unwrap();
            }
        });
}

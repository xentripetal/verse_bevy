use crate::actions::Actions;
use crate::loading::AudioAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioPlugin};

pub struct InternalAudioPlugin;

// This plugin is responsible to controll the game audio
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(AudioPlugin)
            .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(start_music.system()))
            .add_system_set(SystemSet::on_exit(GameState::Menu).with_system(stop_music.system()))
            .add_system_set(
                SystemSet::on_enter(GameState::Playing).with_system(start_audio.system()),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(control_flying_sound.system()),
            );
    }
}

fn start_music(audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
    audio.play_looped(audio_assets.music.clone());
}

fn stop_music(audio: Res<Audio>) {
    audio.stop();
}

fn start_audio(audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
    audio.play_looped(audio_assets.flying.clone());
    audio.pause();
}

fn control_flying_sound(actions: Res<Actions>, audio: Res<Audio>) {
    if actions.player_movement.is_some() {
        audio.resume();
    } else {
        audio.pause()
    }
}

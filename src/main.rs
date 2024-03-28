mod plugins;
mod settings;

use bevy::{app::PluginGroupBuilder, prelude::*, window::WindowResolution};
use plugins::background::BackgroundPlugin;
use plugins::game_over::DeathEvent;
use plugins::game_over::GameOverPlugin;
use plugins::player::PlayerPlugin;
use plugins::pipe::PipePlugin;
use plugins::score::ScorePlugin;
use settings::Settings;

fn main() {
    App::new()
        .add_plugins((
            default_plugins(),
            BackgroundPlugin,
            PlayerPlugin,
            GameOverPlugin,
            PipePlugin,
            ScorePlugin,
        ))
        .add_systems(Startup, setup)
        .init_resource::<Settings>()
        .add_event::<DeathEvent>()
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn default_plugins() -> PluginGroupBuilder {
    DefaultPlugins
        .set(ImagePlugin::default_nearest())
        .set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(576., 1024.).with_scale_factor_override(1.),
                ..default()
            }),
            ..Default::default()
        })
}

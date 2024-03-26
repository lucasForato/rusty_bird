mod plugins;
mod settings;

use bevy::{app::PluginGroupBuilder, prelude::*, window::WindowResolution};
use plugins::background::BackgroundPlugin;
use plugins::game_over::DeathEvent;
use plugins::game_over::GameOverPlugin;
use plugins::player::PlayerPlugin;
use plugins::pipe::PipePlugin;

use settings::player::PlayerSettings;
use settings::background::BackgroundSettings;
use settings::pipe::PipeSettings;
use settings::game_over::GameOverSettings;

fn main() {
    App::new()
        .add_plugins((
            default_plugins(),
            BackgroundPlugin,
            PlayerPlugin,
            GameOverPlugin,
            PipePlugin,
        ))
        .add_systems(Startup, setup)
        .init_resource::<PlayerSettings>()
        .init_resource::<BackgroundSettings>()
        .init_resource::<PipeSettings>()
        .init_resource::<GameOverSettings>()
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

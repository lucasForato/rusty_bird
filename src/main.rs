use bevy::{app::PluginGroupBuilder, prelude::*, window::WindowResolution};

mod constants;

mod plugins;
use plugins::background::BackgroundPlugin;
use plugins::game_over::GameOverPlugin;
use plugins::player::PlayerPlugin;
use plugins::game_over::DeathEvent;

fn main() {
    App::new()
        .add_plugins((
            default_plugins(),
            BackgroundPlugin,
            PlayerPlugin,
            GameOverPlugin,
        ))
        .add_systems(Startup, setup)
        .add_event::<DeathEvent>()
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn window_plugin() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(576., 1024.).with_scale_factor_override(1.),
            ..default()
        }),
        ..Default::default()
    }
}

fn default_plugins() -> PluginGroupBuilder {
    DefaultPlugins
        .set(ImagePlugin::default_nearest())
        .set(window_plugin())
}

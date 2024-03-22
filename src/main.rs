use bevy::{prelude::*, window::WindowResolution};
mod background;
mod player;
mod constants;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(576., 1024.)
                            .with_scale_factor_override(1.),
                        ..default()
                    }),
                    ..Default::default()
                }),
            background::Background,
            player::PlayerPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

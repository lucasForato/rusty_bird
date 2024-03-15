use bevy::prelude::*;
mod player;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, player::PlayerPlugin))
        .add_systems(Startup, add_camera)
        .run();
}

fn add_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

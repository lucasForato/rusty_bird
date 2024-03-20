use bevy::app::Plugin;
use bevy::prelude::*;

pub struct Background;

impl Plugin for Background {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(asset_server: Res<AssetServer>, mut commands: Commands) {
    let texture = asset_server.load("sprites/background-day.png");
    commands.spawn(SpriteBundle {
        texture,
        transform: Transform::from_scale(Vec3::splat(3.0)),
        ..default()
    });
}

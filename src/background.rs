use crate::constants::*;
use bevy::app::Plugin;
use bevy::prelude::*;
use bevy::sprite::Anchor;

pub struct Background;

impl Plugin for Background {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup);
    }
}

fn setup(asset_server: Res<AssetServer>, mut commands: Commands) {
    println!("Setting up background");
    commands.spawn(SpriteBundle {
        texture: asset_server.load("sprites/background-day.png"),
        transform: Transform::from_xyz(0.0, 0.0, BACKGROUND_Z),
        sprite: Sprite {
            custom_size: Some(Vec2::new(288.0 * 2.0, 512.0 * 2.0)),
            ..default()
        },
        ..default()
    });

    commands.spawn(SpriteBundle {
        texture: asset_server.load("sprites/base.png"),
        transform: Transform::from_xyz(0.0, -512.0, BASE_Z),
        sprite: Sprite {
            custom_size: Some(Vec2::new(336.0 * 2.0, 112.0 * 2.0)),
            anchor: Anchor::Center,
            ..default()
        },
        ..default()
    });
}

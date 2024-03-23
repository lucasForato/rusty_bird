use bevy::app::Plugin;
use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::background::DeathEvent;
use crate::constants::GAME_OVER_Z;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            game_over.run_if(|mut player_has_died: EventReader<DeathEvent>| {
                player_has_died.read().count() > 0
            }),
        );
    }
}

fn game_over(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("sprites/gameover.png"),
        transform: Transform::from_xyz(0.0, 0.0, GAME_OVER_Z),
        sprite: Sprite {
            custom_size: Some(Vec2::new(192.0 * 2.0, 42.0 * 2.0)),
            anchor: Anchor::Center,
            ..default()
        },
        ..default()
    });
}

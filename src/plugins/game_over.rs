use crate::plugins::player::Player;
use crate::settings::Settings;
use bevy::app::Plugin;
use bevy::prelude::*;
use bevy::sprite::Anchor;

use super::pipe::Pipe;
use super::score::{Hundreds, Ones, Tens};

pub struct GameOverPlugin;

#[derive(Event)]
pub struct DeathEvent;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            game_over.run_if(|mut player_has_died: EventReader<DeathEvent>| {
                player_has_died.read().count() > 0
            }),
        )
        .add_systems(Update, death_on_ground_system)
        .add_systems(Update, death_on_pipe_system);
    }
}

fn game_over(settings: Res<Settings>, mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("sprites/gameover.png"),
        transform: Transform::from_xyz(0.0, 0.0, settings.game_over_z),
        sprite: Sprite {
            custom_size: Some(Vec2::new(192.0 * 2.0, 42.0 * 2.0)),
            anchor: Anchor::Center,
            ..default()
        },
        ..default()
    });
}

fn death_on_ground_system(
    mut commands: Commands,
    player: Query<(&Transform, Entity), With<Player>>,
    mut event: EventWriter<DeathEvent>,
) {
    for (player_transform, entity) in player.iter() {
        let player_position = player_transform.translation;
        if player_position.y < -390.0 {
            commands.entity(entity).despawn();
            event.send(DeathEvent);
        }
    }
}

fn death_on_pipe_system(
    mut commands: Commands,
    player: Query<(&Transform, &Sprite, Entity), With<Player>>,
    pipe: Query<(&Transform, &Sprite), With<Pipe>>,
    mut writer: EventWriter<DeathEvent>,
) {
    for (player_transform, player_sprite, entity) in player.iter() {
        for (pipe_transform, pipe_sprite) in pipe.iter() {
            let pipe_x_start =
                pipe_transform.translation.x - pipe_sprite.custom_size.unwrap().x / 2.0;
            let pipe_x_end =
                pipe_transform.translation.x + pipe_sprite.custom_size.unwrap().x / 2.0;

            let pipe_y_start =
                pipe_transform.translation.y - pipe_sprite.custom_size.unwrap().y / 2.0;
            let pipe_y_end =
                pipe_transform.translation.y + pipe_sprite.custom_size.unwrap().y / 2.0;

            let player_x_start =
                player_transform.translation.x - player_sprite.custom_size.unwrap().x / 2.0;
            let player_x_end =
                player_transform.translation.x + player_sprite.custom_size.unwrap().x / 2.0;

            let player_y_start =
                player_transform.translation.y - player_sprite.custom_size.unwrap().y / 2.0;
            let player_y_end =
                player_transform.translation.y + player_sprite.custom_size.unwrap().y / 2.0;

            if player_x_start < pipe_x_end
                && player_x_end > pipe_x_start
                && player_y_start < pipe_y_end
                && player_y_end > pipe_y_start
            {
                commands.entity(entity).despawn();
                writer.send(DeathEvent);
            }
        }
    }
}

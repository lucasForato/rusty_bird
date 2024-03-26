use bevy::app::Plugin;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use crate::plugins::player::Player;
use crate::settings::game_over::GameOverSettings;

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
        .add_systems(Update, death_system);
    }
}

fn game_over(
    settings: Res<GameOverSettings>,
    mut commands: Commands, asset_server: Res<AssetServer>) {
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

fn death_system(
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

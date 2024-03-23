use crate::constants::*;
use crate::plugins::player::Player;
use bevy::app::Plugin;
use bevy::prelude::*;
use bevy::sprite::Anchor;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup)
            .add_systems(Update, collision_system);
    }
}

#[derive(Component)]
struct Ground;

#[derive(Event)]
pub struct DeathEvent;

fn setup(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("sprites/background-day.png"),
        transform: Transform::from_xyz(0.0, 0.0, BACKGROUND_Z),
        sprite: Sprite {
            custom_size: Some(Vec2::new(288.0 * 2.0, 512.0 * 2.0)),
            ..default()
        },
        ..default()
    });

    commands.spawn((
        Ground,
        SpriteBundle {
            texture: asset_server.load("sprites/base.png"),
            transform: Transform::from_xyz(0.0, -512.0, BASE_Z),
            sprite: Sprite {
                custom_size: Some(Vec2::new(336.0 * 2.0, 112.0 * 2.0)),
                anchor: Anchor::Center,
                ..default()
            },
            ..default()
        },
    ));
}

fn collision_system(
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

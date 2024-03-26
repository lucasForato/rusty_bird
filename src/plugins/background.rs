use bevy::app::Plugin;
use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::settings::background::BackgroundSettings;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup)
            .add_systems(Update, ground_movement)
            .add_systems(
                Update,
                spawn_ground.run_if(|mut spawn_new_ground: EventReader<SpawnNewGroundEvent>| {
                    spawn_new_ground.read().count() > 0
                }),
            )
            .add_event::<SpawnNewGroundEvent>();
    }
}

#[derive(Component)]
struct Ground;

#[derive(Event)]
struct SpawnNewGroundEvent;

fn setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut writer: EventWriter<SpawnNewGroundEvent>,
    settings: Res<BackgroundSettings>,
) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("sprites/background-day.png"),
        transform: Transform::from_xyz(0.0, 0.0, settings.background_z),
        sprite: Sprite {
            custom_size: Some(Vec2::new(288.0 * 2.0, 512.0 * 2.0)),
            ..default()
        },
        ..default()
    });

    writer.send(SpawnNewGroundEvent);
}

fn ground_movement(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Ground>>,
    mut writer: EventWriter<SpawnNewGroundEvent>,
) {
    for mut transform in query.iter_mut() {
        transform.translation.x -= 100.0 * time.delta_seconds();
        if transform.translation.x <= -1052.0 {
            writer.send(SpawnNewGroundEvent);
        }
    }
}

fn spawn_ground(
    mut commands: Commands,
    query: Query<Entity, With<Ground>>,
    asset_server: Res<AssetServer>,
    settings: Res<BackgroundSettings>
) {
    commands.spawn((
        Ground,
        SpriteBundle {
            texture: asset_server.load("sprites/base.png"),
            transform: Transform::from_xyz(-288., -412.0, settings.base_z),
            sprite: Sprite {
                custom_size: Some(Vec2::new(672.0 * 2.0, 112.0 * 2.0)),
                anchor: Anchor::TopLeft,
                ..default()
            },
            ..default()
        },
    ));
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

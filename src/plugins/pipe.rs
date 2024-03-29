use crate::settings::Settings;
use bevy::app::Plugin;
use bevy::prelude::*;
use rand::prelude::*;

use super::score::IncreaseScore;

pub struct PipePlugin;

enum Surface {
    GROUND,
    SKY,
}

#[derive(Event)]
struct SpawnNewPipeEvent {
    height: f32,
    surface: Surface,
}

#[derive(Component)]
struct SpawnTimer {
    timer: Timer,
}

#[derive(Component)]
pub struct Pipe;

#[derive(Component)]
pub struct Scorable;

impl Plugin for PipePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup)
            .add_systems(
                Update,
                spawn_pipe.run_if(|mut spawn_new_pipe: EventReader<SpawnNewPipeEvent>| {
                    spawn_new_pipe.read().count() > 0
                }),
            )
            .add_systems(Update, timer_system)
            .add_systems(
                Update,
                (move_pipe, pipe_reached_end_of_screen, pipe_reached_player),
            )
            .add_event::<SpawnNewPipeEvent>();
    }
}

fn setup(mut commands: Commands, mut writer: EventWriter<SpawnNewPipeEvent>) {
    writer.send(SpawnNewPipeEvent {
        height: get_pipe_height(),
        surface: get_pipe_surface(),
    });

    commands.spawn(SpawnTimer {
        timer: Timer::from_seconds(2.0, TimerMode::Repeating),
    });
}

fn spawn_pipe(
    mut commands: Commands,
    mut pipe_props: EventReader<SpawnNewPipeEvent>,
    settings: Res<Settings>,
    asset_server: Res<AssetServer>,
) {
    for pipe_prop in pipe_props.read() {
        match pipe_prop.surface {
            Surface::GROUND => {
                let pipe_height = pipe_prop.height + -732.0;
                commands.spawn((
                    Pipe,
                    Scorable,
                    SpriteBundle {
                        texture: asset_server.load("sprites/pipe-green.png"),
                        transform: Transform::from_xyz(400.0, pipe_height, settings.pipe_z),
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(52.0 * 2.0, 320.0 * 2.0)),
                            ..default()
                        },
                        ..default()
                    },
                ));
            }
            Surface::SKY => {
                let pipe_height = pipe_prop.height + 200.0;
                commands.spawn((
                    Pipe,
                    Scorable,
                    SpriteBundle {
                        texture: asset_server.load("sprites/pipe-green.png"),
                        transform: Transform::from_xyz(400.0, pipe_height, settings.pipe_z),
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(52.0 * 2.0, 320.0 * 2.0)),
                            flip_y: true,
                            ..default()
                        },
                        ..default()
                    },
                ));
            }
        }
    }
}

fn get_pipe_height() -> f32 {
    let y: f32 = generate_random_number();
    y * 500.0
}

fn generate_random_number() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

fn get_pipe_surface() -> Surface {
    if generate_random_number() < 0.5 {
        Surface::SKY
    } else {
        Surface::GROUND
    }
}

fn move_pipe(time: Res<Time>, mut query: Query<&mut Transform, With<Pipe>>) {
    for mut transform in query.iter_mut() {
        transform.translation.x -= 100.0 * time.delta_seconds();
    }
}

fn timer_system(
    mut writer: EventWriter<SpawnNewPipeEvent>,
    mut spawn_timers: Query<&mut SpawnTimer>,
    time: Res<Time>,
) {
    for mut spawn_timer in spawn_timers.iter_mut() {
        spawn_timer.timer.tick(time.delta());
        if spawn_timer.timer.finished() {
            writer.send(SpawnNewPipeEvent {
                height: get_pipe_height(),
                surface: get_pipe_surface(),
            });
        }
    }
}

fn pipe_reached_end_of_screen(
    mut commands: Commands,
    mut pipes: Query<(Entity, &Transform), With<Pipe>>,
) {
    for (entity, transform) in pipes.iter_mut() {
        if transform.translation.x < -400.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn pipe_reached_player(
    mut pipes: Query<(&Transform, Entity), With<Scorable>>,
    mut writer: EventWriter<IncreaseScore>,
    mut commands: Commands,
) {
    for (transform, entity) in pipes.iter_mut() {
        if transform.translation.x < -20.0 {
            writer.send(IncreaseScore);
            commands.entity(entity).remove::<Scorable>();
        }
    }
}

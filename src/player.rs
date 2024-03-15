use std::hint;

use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use bevy::time::Stopwatch;

const PLAYER_SPRITE: &str = "sprites/redbird-midflap.png";
const SPEED: f32 = 300.;

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Movement {
    direction: Direction,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, render_player_system)
            .add_systems(Update, input_system)
            .add_systems(Update, movement_system)
            .add_systems(Update, timer_system);
    }
}

fn render_player_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Player,
        SpriteBundle {
            texture: asset_server.load(PLAYER_SPRITE),
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        },
        Movement {
            direction: Direction::Down,
        },
    ));
}

fn input_system(
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut query: Query<&mut Movement, With<Player>>,
) {
    for event in keyboard_input_events.read() {
        if event.key_code == KeyCode::Space {
            for mut movement in query.iter_mut() {
                movement.direction = Direction::Up;
            }
        }
    }
}

fn timer_system(mut query: Query<&mut Movement, Changed<Movement>>) {
    let stopwatch = Stopwatch::new();
    while stopwatch.elapsed().as_secs() < 1 {
        println!("time elapsed: {:?}", stopwatch.elapsed().as_secs());
        hint::spin_loop();
    }
    for mut movement in query.iter_mut() {
        movement.direction = Direction::Down;
    }
}

fn movement_system(time: Res<Time>, mut query: Query<(&mut Transform, &Movement), With<Player>>) {
    for (mut transform, movement) in query.iter_mut() {
        match movement.direction {
            Direction::Up => {
                transform.translation.y += SPEED * time.delta_seconds();
            }
            Direction::Down => {
                transform.translation.y -= SPEED * time.delta_seconds();
            }
        }
        // transform.translation.y -= GRAVITY_SPEED * time.delta_seconds();
    }
}

use crate::constants::*;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use bevy::sprite::Anchor;

pub struct PlayerPlugin;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct JumpTimer {
    timer: Timer,
}

enum Direction {
    Up,
    Down,
}

#[derive(Component)]
struct MovementDirection {
    direction: Direction,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(
            Update,
            (animate_sprite_system, input_system, jump_system).chain(),
        );
    }
}

fn animate_sprite_system(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}

fn setup(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    let layout = TextureAtlasLayout::from_grid(
        Vec2::new(PLAYER_SPRITE.x, PLAYER_SPRITE.y),
        3,
        1,
        None,
        None,
    );
    let animation_indices = AnimationIndices { first: 0, last: 2 };

    commands.spawn((
        Player,
        SpriteSheetBundle {
            texture: asset_server.load("sprites/red_bird_animation.png"),
            atlas: TextureAtlas {
                layout: texture_atlas_layouts.add(layout),
                index: animation_indices.first,
            },
            transform: Transform::from_xyz(0.0, 0.0, 100.0),
            sprite: Sprite {
                custom_size: Some(PLAYER_SPRITE_DIMENSIONS),
                anchor: Anchor::Center,
                ..default()
            },
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        JumpTimer {
            timer: Timer::from_seconds(0.5, TimerMode::Once),
        },
        MovementDirection {
            direction: Direction::Down,
        },
    ));
}

fn input_system(
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut query: Query<(&mut MovementDirection, &mut JumpTimer), With<Player>>,
) {
    for event in keyboard_input_events.read() {
        if event.key_code == KeyCode::Space {
            for (mut movement_direction, mut jump_timer) in query.iter_mut() {
                movement_direction.direction = Direction::Up;
                jump_timer.timer.reset();
            }
        }
    }
}

fn jump_system(
    time: Res<Time>,
    mut query: Query<(&mut JumpTimer, &mut MovementDirection, &mut Transform), With<Player>>,
) {
    for (mut jump_timer, mut movement_direction, mut transform) in query.iter_mut() {
        jump_timer.timer.tick(time.delta());

        match movement_direction.direction {
            Direction::Up => {
                let force: f32 = jump_timer.timer.elapsed_secs() * 10.0;
                let velocity = (PLAYER_SPEED - force) * 150.0;
                let formula: f32 = velocity * time.delta_seconds();
                transform.translation.y += formula;
            }
            Direction::Down => {
                let velocity = PLAYER_SPEED * 120.0;
                let formula: f32 = velocity * time.delta_seconds();
                transform.translation.y -= formula;
            }
        }

        if jump_timer.timer.finished() {
            movement_direction.direction = Direction::Down;
        }
    }
}

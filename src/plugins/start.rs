use crate::settings::{GameState, Settings};
use bevy::{input::keyboard::KeyboardInput, prelude::*};

pub struct StartPlugin;

#[derive(Component)]
pub struct Start;

impl Plugin for StartPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, start_input.run_if(in_state(GameState::MainMenu)));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, settings: Res<Settings>) {
    commands.spawn((
        Start,
        SpriteBundle {
            texture: asset_server.load("sprites/message.png"),
            transform: Transform::from_xyz(0.0, 0.0, settings.start_z),
            sprite: Sprite {
                custom_size: Some(Vec2::new(184.0 * 2.0, 152.0 * 2.0)),
                ..default()
            },
            ..default()
        },
    ));
}

fn start_input(
    mut commands: Commands,
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut next_state: ResMut<NextState<GameState>>,
    query: Query<(&Start, Entity)>,
) {
    for event in keyboard_input_events.read() {
        if event.key_code == KeyCode::Space {
            info!("Starting gameplay");
            next_state.set(GameState::Playing);
            for (_, entity) in query.iter() {
                commands.entity(entity).despawn()
            }
        }
    }
}

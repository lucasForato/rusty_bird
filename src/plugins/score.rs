use bevy::prelude::*;

use crate::settings::Settings;

pub struct ScorePlugin;

#[derive(Event)]
pub struct IncreaseScore;

#[derive(Component)]
struct Score {
    pub score: i32,
}

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_event::<IncreaseScore>()
            .add_systems(
                Update,
                increase_score_system
                    .run_if(|mut events: EventReader<IncreaseScore>| events.read().count() > 0),
            );
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, settings: Res<Settings>) {
    commands.spawn((
        Score { score: 0 },
        SpriteBundle {
            texture: asset_server.load("sprites/0.png"),
            transform: Transform::from_xyz(-250.0, 450.0, settings.score_z),
            sprite: Sprite {
                custom_size: Some(Vec2::new(24.0 * 2.0, 36.0 * 2.0)),
                ..default()
            },
            ..default()
        },
    ));
}

fn increase_score_system(
    mut query: Query<(&mut Score, Entity), With<Score>>,
    asset_server: Res<AssetServer>,
    commands: Commands,
) {
    for (mut score, mut texture) in query.iter_mut() {
        score.score += 1;

    }
}

use bevy::prelude::*;

use crate::settings::Settings;

use super::player::Player;

pub struct ScorePlugin;

#[derive(Resource)]
pub struct Numbers {
    pub n0: Handle<Image>,
    pub n1: Handle<Image>,
    pub n2: Handle<Image>,
    pub n3: Handle<Image>,
    pub n4: Handle<Image>,
    pub n5: Handle<Image>,
    pub n6: Handle<Image>,
    pub n7: Handle<Image>,
    pub n8: Handle<Image>,
    pub n9: Handle<Image>,
}

#[derive(Event)]
pub struct IncreaseScore;

#[derive(Component)]
pub struct Ones {
    pub score: i32,
}

#[derive(Component)]
pub struct Tens {
    pub score: i32,
}

#[derive(Component)]
pub struct Hundreds {
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
    commands.insert_resource(Numbers {
        n0: asset_server.load("sprites/0.png"),
        n1: asset_server.load("sprites/1.png"),
        n2: asset_server.load("sprites/2.png"),
        n3: asset_server.load("sprites/3.png"),
        n4: asset_server.load("sprites/4.png"),
        n5: asset_server.load("sprites/5.png"),
        n6: asset_server.load("sprites/6.png"),
        n7: asset_server.load("sprites/7.png"),
        n8: asset_server.load("sprites/8.png"),
        n9: asset_server.load("sprites/9.png"),
    });

    commands.spawn((
        Ones { score: 0 },
        SpriteBundle {
            texture: asset_server.load("sprites/0.png"),
            transform: Transform::from_xyz(-150.0, 450.0, settings.score_z),
            sprite: Sprite {
                custom_size: Some(Vec2::new(24.0 * 2.0, 36.0 * 2.0)),
                ..default()
            },
            ..default()
        },
    ));

    commands.spawn((
        Tens { score: 0 },
        SpriteBundle {
            texture: asset_server.load("sprites/0.png"),
            transform: Transform::from_xyz(-200.0, 450.0, settings.score_z),
            sprite: Sprite {
                custom_size: Some(Vec2::new(24.0 * 2.0, 36.0 * 2.0)),
                ..default()
            },
            ..default()
        },
    ));

    commands.spawn((
        Tens { score: 0 },
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
    player: Query<&Player>,
    mut ones: Query<(&mut Handle<Image>, &mut Ones), Without<Tens>>,
    mut tens: Query<(&mut Handle<Image>, &mut Tens), Without<Ones>>,
    mut hundreds: Query<(&mut Handle<Image>, &mut Hundreds), (Without<Ones>, Without<Tens>)>,
    numbers: ResMut<Numbers>,
) {
    let mut increase_tens = false;
    let mut increase_hundreds = false;

    if player.iter().count() == 0 {
        return;
    }

    // Process ones
    for (mut handle, mut ones) in ones.iter_mut() {
        if ones.score == 9 {
            ones.score = 0;
            increase_tens = true;
        } else {
            ones.score += 1;
        }
        *handle = get_number(ones.score, &numbers);
    }

    // Process tens
    for (mut handle, mut tens) in tens.iter_mut() {
        if increase_tens {
            if tens.score == 9 {
                tens.score = 0;
                increase_hundreds = true;
            } else {
                tens.score += 1;
            }
            increase_tens = false;
        }
        *handle = get_number(tens.score, &numbers);
    }

    // Process hundreds
    for (mut handle, mut hundreds) in hundreds.iter_mut() {
        if increase_hundreds {
            hundreds.score += 1;
            increase_hundreds = false;
        }
        *handle = get_number(hundreds.score, &numbers);
    }
}

fn get_number(num: i32, numbers: &Numbers) -> Handle<Image> {
    match num {
        0 => numbers.n0.clone(),
        1 => numbers.n1.clone(),
        2 => numbers.n2.clone(),
        3 => numbers.n3.clone(),
        4 => numbers.n4.clone(),
        5 => numbers.n5.clone(),
        6 => numbers.n6.clone(),
        7 => numbers.n7.clone(),
        8 => numbers.n8.clone(),
        9 => numbers.n9.clone(),
        _ => numbers.n0.clone(),
    }
}

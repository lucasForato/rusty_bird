use bevy::prelude::*;

#[derive(Resource)]
pub struct GameOverSettings {
    pub game_over_z: f32,
}

impl Default for GameOverSettings {
    fn default() -> Self {
        Self {
            game_over_z: 100.0,
                    
        }
    }
}

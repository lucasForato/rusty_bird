use bevy::prelude::*;

#[derive(Resource)]
pub struct BackgroundSettings {
    pub base_z: f32,
    pub background_z: f32, 
}

impl Default for BackgroundSettings {
    fn default() -> Self {
        Self {
            base_z: 10.0,
            background_z: 1.0,
                    
        }
    }
}

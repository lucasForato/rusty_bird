use bevy::prelude::*;

#[derive(Resource)]
pub struct PipeSettings {
    pub pipe_z: f32,
}

impl Default for PipeSettings {
    fn default() -> Self {
        Self {
            pipe_z: 50.0
                    
        }
    }
}

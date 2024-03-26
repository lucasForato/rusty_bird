use bevy::prelude::*;

#[derive(Resource)]
pub struct PlayerSettings {
    pub speed: f32,
    pub sprite: Vec3, 
    pub sprite_dimensions: Vec2,
}

impl Default for PlayerSettings {
    fn default() -> Self {
        Self {
            speed: 3.0,
            sprite: Vec3::new(34.0, 24.0, 100.0),
            sprite_dimensions: Vec2::new(68.0, 48.0),
        }
    }
}

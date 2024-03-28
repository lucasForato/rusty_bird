use bevy::prelude::*;

#[derive(Resource)]
pub struct Settings {
    pub base_z: f32,
    pub background_z: f32, 
    pub game_over_z: f32,
    pub pipe_z: f32,
    pub speed: f32,
    pub sprite: Vec3, 
    pub sprite_dimensions: Vec2,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            base_z: 10.0,
            background_z: 1.0,
            speed: 3.0,
            sprite: Vec3::new(34.0, 24.0, 100.0),
            sprite_dimensions: Vec2::new(68.0, 48.0),
            game_over_z: 100.0,
            pipe_z: 5.0,
        }
    }
}

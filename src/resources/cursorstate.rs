use bevy::ecs::system::Resource;
use bevy::math::{Vec2, Vec3};

#[derive(Resource)]
pub struct CursorState {
    pub previous_position: Vec2,
    pub grid_position: Vec3,
}

impl Default for CursorState {
    fn default() -> Self {
        Self {
            previous_position: Vec2::ZERO,
            grid_position: Vec3::ZERO,
        }
    }
}
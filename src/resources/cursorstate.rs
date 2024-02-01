use bevy::ecs::system::Resource;
use bevy::math::Vec2;

#[derive(Resource)]
pub struct CursorState {
    pub previous_position: Vec2,
}

impl Default for CursorState {
    fn default() -> Self {
        Self {
            previous_position: Vec2::ZERO,
        }
    }
}
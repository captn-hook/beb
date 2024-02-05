use bevy::prelude::*;
use crate::resources::cursorstate::CursorState;
use crate::settings::input::InputSystem;
use crate::objects::cube::new_cube_bundle;
use crate::resources::blockplaced::BlockPlaced;

pub fn place_block(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    input_system: Res<InputSystem>,
    cursor_state: Res<CursorState>,
    mut place_delay: ResMut<BlockPlaced>,
) {
    if input_system.interact_1 && place_delay.can_place {
        new_cube_bundle(cursor_state.grid_position, &mut meshes, &mut materials, &mut commands);
        place_delay.placed();
    }
}
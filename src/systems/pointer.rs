use bevy::prelude::*;
use bevy_mod_picking::pointer::PointerInteraction;
use crate::resources::cursorstate::CursorState;

pub fn draw_pointer(
    pointer: Query<&PointerInteraction>,
    mut gizmos: Gizmos,
    mut cursor_state: ResMut<CursorState>,
) {
    //impl PointerInteraction
    //pub fn get_nearest_hit(&self) -> Option<&(Entity, HitData)>

    if let Some((_, hit)) = pointer.iter().next().and_then(|p| p.get_nearest_hit()) {
        if let Some(normal) = hit.normal {
            let position = hit.position.unwrap_or_default();
            let distance = position + normal * 0.1;
            gizmos.circle(distance, normal, 0.1, Color::RED);
            //create a Transform::IDENTITY for the box, defaults to scales 1 and rotations 0
            let transform = Transform::from_translation(distance.round()); // <- Round the position to snap to the grid.
            cursor_state.grid_position = distance.round();
            // Draw a wireframe box
            gizmos.cuboid( transform, Color::RED);
        }
    }
}
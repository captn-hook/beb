use bevy::{
    prelude::*,
    input::mouse::MouseWheel,
};
use crate::resources::cursorstate::CursorState;
use crate::settings::input::InputSystem;

pub fn camera_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    mut ev_cursor_moved: EventReader<CursorMoved>,
    mut ev_scroll: EventReader<MouseWheel>,
    mut query: Query<(&mut Transform, &Camera)>,
    mut cursor_state: ResMut<CursorState>,
) {
    let mut delta = Vec3::ZERO;
    let mut input_system = InputSystem::new();
    input_system.update(&keyboard_input, &mouse_input);

    for (mut transform, _camera) in query.iter_mut() {
        let mut changed = false;
        for event in ev_cursor_moved.read() {
            if mouse_input.pressed(MouseButton::Right) {
                let delta = event.position - cursor_state.previous_position;
                let sensitivity = 0.001;
                let yaw = Quat::from_rotation_y(-delta.x * sensitivity);
                let pitch = Quat::from_rotation_x(-delta.y * sensitivity);
                transform.rotation = yaw * transform.rotation * pitch;
                changed = true;
            }
            cursor_state.previous_position = event.position;
        }

        let mut sensitivity = 5.0;

        if input_system.speed.slow {
            sensitivity = 15.0;
        }

        for event in ev_scroll.read() {
            delta.z -= event.y * sensitivity;
            changed = true;
        }
        
        if input_system.movement.forward {
            delta.z -= sensitivity;
            changed = true;
        }
        if input_system.movement.backward {
            delta.z += sensitivity;
            changed = true;
        }
        if input_system.movement.left {
            delta.x -= sensitivity;
            changed = true;
        }
        if input_system.movement.right {
            delta.x += sensitivity;
            changed = true;
        }
        if input_system.movement.up {
            delta.y += sensitivity;
            changed = true;
        }
        if input_system.movement.down {
            delta.y -= sensitivity;
            changed = true;
        }
        
        if changed {
            let translation = transform.local_x() * delta.x + transform.local_y() * delta.y + transform.local_z() * delta.z;
            transform.translation += translation * time.delta_seconds();
        }
    }
}
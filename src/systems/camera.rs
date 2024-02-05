use bevy::prelude::*;
use crate::settings::input::InputSystem;

pub fn camera_movement(
    time: Res<Time>,
    input_system: Res<InputSystem>,
    mut query: Query<(&mut Transform, &Camera)>,
) {
    let mut delta = Vec3::ZERO;

    let mut sensitivity = 0.005;

    for (mut transform, _camera) in query.iter_mut() {
        let mut changed = false;
        if input_system.interact_3 {
            let yaw = Quat::from_rotation_y(-input_system.cursor_moved_delta.x * sensitivity);
            let pitch = Quat::from_rotation_x(-input_system.cursor_moved_delta.y * sensitivity);
            transform.rotation = yaw * transform.rotation * pitch;
            changed = true;
        }
    

        if input_system.speed.slow {
            sensitivity = 15.0;
        }

        if input_system.mouse_wheel_delta != 0.0 {
            delta.z -= input_system.mouse_wheel_delta * sensitivity;
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
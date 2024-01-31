//! Demonstrates how to spawn and control a virtual pointer, useful for integration testing or
//! something like a gamepad-controlled software pointer.

use bevy::{
    prelude::*,
    input::mouse::MouseWheel,
};
use bevy_mod_picking::prelude::*;

#[derive(Resource)]
struct CursorState {
    previous_position: Vec2,
}

impl Default for CursorState {
    fn default() -> Self {
        Self {
            previous_position: Vec2::ZERO,
        }
    }
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(low_latency_window_plugin()),
            DefaultPickingPlugins,
            bevy_egui::EguiPlugin, // Nicer pointer debug overlay, useful for this example.
        ))
        .insert_resource(CursorState::default())
        .add_systems(Startup, setup)
        .add_systems(Update, draw_pointer)
        .add_systems(Update, camera_movement)
        .run();
}


fn draw_pointer(
    pointer: Query<&PointerInteraction>,
    mut gizmos: Gizmos,
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
            // Draw a wireframe box
            gizmos.cuboid( transform, Color::RED);
        }
    }
}

//camera wasd and mouse pan (right click) / zoom
fn camera_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    mut ev_cursor_moved: EventReader<CursorMoved>,
    mut ev_scroll: EventReader<MouseWheel>,
    mut query: Query<(&mut Transform, &Camera)>,
    mut cursor_state: ResMut<CursorState>,
) {
    let mut delta = Vec3::ZERO;
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

        if keyboard_input.pressed(KeyCode::ShiftLeft) {
            sensitivity = 15.0;
        }

        for event in ev_scroll.read() {
            delta.z -= event.y * sensitivity;
            changed = true;
        }
        
        if keyboard_input.pressed(KeyCode::W) {
            delta.z -= sensitivity;
            changed = true;
        }
        if keyboard_input.pressed(KeyCode::S) {
            delta.z += sensitivity;
            changed = true;
        }
        if keyboard_input.pressed(KeyCode::A) {
            delta.x -= sensitivity;
            changed = true;
        }
        if keyboard_input.pressed(KeyCode::D) {
            delta.x += sensitivity;
            changed = true;
        }
        if keyboard_input.pressed(KeyCode::Space) {
            delta.y += sensitivity;
            changed = true;
        }
        if keyboard_input.pressed(KeyCode::ControlLeft) {
            delta.y -= sensitivity;
            changed = true;
        }
        
        if changed {
            let translation = transform.local_x() * delta.x + transform.local_y() * delta.y + transform.local_z() * delta.z;
            transform.translation += translation * time.delta_seconds();
        }
    }
}


/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    // cube
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::WHITE.into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        PickableBundle::default(), // <- Makes the mesh pickable.
    ));

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, -4.0),
        ..default()
    });

    // camera
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(3.0, 3.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    },));
}
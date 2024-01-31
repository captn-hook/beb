//! Demonstrates how to spawn and control a virtual pointer, useful for integration testing or
//! something like a gamepad-controlled software pointer.

use bevy::{
    prelude::*,
};
use bevy_mod_picking::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(low_latency_window_plugin()),
            DefaultPickingPlugins,
            bevy_egui::EguiPlugin, // Nicer pointer debug overlay, useful for this example.
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, draw_pointer)
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
            gizmos.circle(position + normal * 0.01, normal, 0.1, Color::RED);
        }
    }
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
      // plane
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane::from_size(5.0))),
            material: materials.add(Color::WHITE.into()),
            ..default()
        },
        PickableBundle::default(), // <- Makes the mesh pickable.
    ));

    // cube
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::WHITE.into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
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
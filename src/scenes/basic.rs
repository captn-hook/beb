use crate::objects::cube::new_cube_bundle;
use bevy::prelude::*;

/// set up a simple 3D scene
pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // cube
    commands.spawn(new_cube_bundle(Vec3::new(0.0, 0.0, 0.0), &mut meshes, &mut materials));
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

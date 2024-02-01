use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

pub trait Block {
    fn mesh(&self, meshes: &mut ResMut<Assets<Mesh>>) -> Handle<Mesh>;
    fn material(&self, materials: &mut ResMut<Assets<StandardMaterial>>) -> Handle<StandardMaterial>;
}

pub struct Cube {
    pub size: f32,
    pub color: Color,
}

impl Block for Cube {
    fn mesh(&self, meshes: &mut ResMut<Assets<Mesh>>) -> Handle<Mesh> {
        meshes.add(Mesh::from(shape::Cube { size: self.size }))
    }

    fn material(&self, materials: &mut ResMut<Assets<StandardMaterial>>) -> Handle<StandardMaterial> {
        materials.add(self.color.into())
    }
}

pub fn new_cube_bundle(
    location: Vec3,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> (PbrBundle, PickableBundle) {
    let cube = Cube {
        size: 1.0,
        color: Color::rgb(0.8, 0.7, 0.6),
    };

    let mesh = cube.mesh(meshes);
    let material = cube.material(materials);

    (
        PbrBundle {
            mesh,
            material,
            transform: Transform::from_translation(location),
            ..Default::default()
        },
        PickableBundle::default(),
    )
}
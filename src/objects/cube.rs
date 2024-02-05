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
    commands: &mut Commands,
) {
    let cube = Cube {
        size: 1.0,
        color: Color::rgb(0.8, 0.7, 0.6),
    };

    let mesh = cube.mesh(meshes);
    let material = cube.material(materials);

    let pbr_bundle = PbrBundle {
        mesh,
        material,
        transform: Transform::from_translation(location),
        ..Default::default()
    };

    let pickable_bundle = PickableBundle::default();

    commands.spawn((
        pbr_bundle,
        pickable_bundle,
        // Despawn an entity when clicked:
        On::<Pointer<Click>>::target_commands_mut(|_click, target_commands| {
            target_commands.despawn();
        }),
    ));
}
mod systems {
    pub mod pointer;
    pub mod camera;
    pub mod place_block;
}

mod resources {
    pub mod cursorstate;
    pub mod blockplaced;
}

mod scenes {
    pub mod basic;
}

mod settings {
    pub mod input;
}

mod objects {
    pub mod cube;
}

use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use systems::pointer::draw_pointer;
use systems::camera::camera_movement;
use resources::cursorstate::CursorState;
use scenes::basic::setup;
use systems::place_block::place_block;
use settings::input::{InputSystem, update_input_system};
use resources::blockplaced::BlockPlaced;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(low_latency_window_plugin()),
            DefaultPickingPlugins,
            bevy_egui::EguiPlugin, // Nicer pointer debug overlay, useful for this example.
        ))
        .insert_resource(CursorState::default())
        .insert_resource(InputSystem::new())
        .insert_resource(BlockPlaced::new(0.25))
        .add_systems(Startup, setup)
        .add_systems(Update, update_input_system)
        .add_systems(Update, draw_pointer)
        .add_systems(Update, camera_movement)
        .add_systems(Update, place_block)
        .run();
}

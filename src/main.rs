mod systems {
    pub mod pointer;
    pub mod camera;
}

mod resources {
    pub mod cursorstate;
}

mod scenes {
    pub mod basic;
}

mod settings {
    pub mod input;
}

use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use systems::pointer::draw_pointer;
use systems::camera::camera_movement;
use resources::cursorstate::CursorState;
use scenes::basic::setup;

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

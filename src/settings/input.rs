use bevy::{
    prelude::*,
    input::mouse::MouseWheel,
};
use bevy::input::keyboard::KeyCode;

#[derive(Resource)]
pub struct InputSystem {
    pub movement: Movement,
    pub speed: Speed,
    pub interact_1: bool,
    pub interact_2: bool,
    pub interact_3: bool,
    pub cursor_position: Vec2,
    pub cursor_moved_delta: Vec2,
    pub mouse_wheel_delta: f32,
}

pub struct Movement {
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
}

pub struct Speed {
    pub fast: bool,
}

impl InputSystem {
    pub fn new() -> Self {
        Self {
            movement: Movement {
                forward: false,
                backward: false,
                left: false,
                right: false,
                up: false,
                down: false,
            },
            speed: Speed {
                fast: false,
            },
            interact_1: false,
            interact_2: false,
            interact_3: false,
            cursor_position: Vec2::ZERO,
            cursor_moved_delta: Vec2::ZERO,
            mouse_wheel_delta: 0.0,
        }
    }

    pub fn update(&mut self, keyboard_input: &Res<Input<KeyCode>>, mouse_button_input: &Res<Input<MouseButton>>, cursor_input: &mut EventReader<CursorMoved>, scroll: &mut EventReader<MouseWheel>) {
        self.movement.forward = keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up);
        self.movement.backward = keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down);
        self.movement.left = keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left);
        self.movement.right = keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);
        self.movement.up = keyboard_input.pressed(KeyCode::Space);
        self.movement.down = keyboard_input.pressed(KeyCode::ControlLeft) || keyboard_input.pressed(KeyCode::ControlRight);
    
        self.speed.fast = keyboard_input.pressed(KeyCode::ShiftLeft) || keyboard_input.pressed(KeyCode::ShiftRight);
    
        self.interact_1 = mouse_button_input.pressed(MouseButton::Left);
        self.interact_2 = mouse_button_input.pressed(MouseButton::Right);
        self.interact_3 = mouse_button_input.pressed(MouseButton::Middle);

        for event in cursor_input.read() {
            self.cursor_moved_delta = event.position - self.cursor_position;
            self.cursor_position = event.position;
        }

        self.mouse_wheel_delta = 0.0;
        for event in scroll.read() {
            self.mouse_wheel_delta += event.y;
        }
    }
}

pub fn update_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut cursor_input: EventReader<CursorMoved>,
    mut scroll: EventReader<MouseWheel>,
    mut input_system: ResMut<InputSystem>,
) {
    input_system.update(&keyboard_input, &mouse_button_input, &mut cursor_input, &mut scroll);
}
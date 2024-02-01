use bevy::prelude::*;
use bevy::input::keyboard::KeyCode;

pub struct InputSystem {
    pub movement: Movement,
    pub speed: Speed,
    pub pan: bool,
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
    pub slow: bool,
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
                slow: false,
                fast: false,
            },
            pan: false,
        }
    }

    pub fn update(&mut self, keyboard_input: &Res<Input<KeyCode>>, mouse_button_input: &Res<Input<MouseButton>>) {
        self.movement.forward = keyboard_input.pressed(KeyCode::W);
        self.movement.backward = keyboard_input.pressed(KeyCode::S);
        self.movement.left = keyboard_input.pressed(KeyCode::A);
        self.movement.right = keyboard_input.pressed(KeyCode::D);
        self.movement.up = keyboard_input.pressed(KeyCode::Space);
        self.movement.down = keyboard_input.pressed(KeyCode::ControlLeft);
    
        self.speed.slow = keyboard_input.pressed(KeyCode::ShiftLeft);
        self.speed.fast = keyboard_input.pressed(KeyCode::ShiftRight);
    
        self.pan = mouse_button_input.pressed(MouseButton::Right);
    }
}
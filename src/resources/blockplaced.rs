use bevy::prelude::*;

#[derive(Resource)]
pub struct BlockPlaced {
    timer: Timer,
    pub can_place: bool,
}

impl BlockPlaced {
    pub fn new(cooldown: f32) -> Self {
        Self {
            timer: Timer::from_seconds(cooldown, TimerMode::Once),
            can_place: true,
        }
    }

    pub fn placed(&mut self) {
        self.can_place = false;
        self.timer.reset();
    }

    pub fn update(&mut self, time: &Time) {
        if !self.can_place {
            if self.timer.tick(time.delta()).just_finished() {
                self.can_place = true;
            }
        }
    }
}

pub fn block_placed_update_system(mut block_placed: ResMut<BlockPlaced>, time: Res<Time>) {
    block_placed.update(&time);
}
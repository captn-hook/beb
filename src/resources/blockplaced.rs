use bevy::prelude::*;

#[derive(Resource)]
pub struct BlockPlaced {
    last_placed: Option<Timer>,
    cooldown: Timer,
}

impl BlockPlaced {
    pub fn new(cooldown: f32) -> Self {
        Self {
            last_placed: None,
            cooldown: Timer::from_seconds(cooldown, TimerMode::Once),
        }
    }

    pub fn can_place(&mut self, time: Res<Time>) -> bool {
        if let Some(last_placed) = &mut self.last_placed {
            last_placed.tick(time.delta());
            if last_placed.finished() {
                self.last_placed = None;
                return true;
            }
        }
    
        self.last_placed.is_none()
    }

    pub fn placed_block(&mut self) {
        self.last_placed = Some(self.cooldown.clone());
    }
}
use macroquad::prelude::*;
use crate::utils::V2;
use crate::animation::Animation;

pub enum ActionType {
    Walk,
    Attack,
    Death
}

pub enum PlayerData {
    Position,
    Hitbox,
    State,
    Animation
}

pub(crate) struct Player {
    pub position: V2,
    pub hitbox: f32,
    pub state: ActionType,
    pub walk: Animation,
    pub attack: Animation,
    pub death: Animation
}

impl Player {
    pub fn new(position: V2, hitbox: f32, walk: Animation, attack: Animation, death: Animation) -> Self {
        Player {
            position,
            hitbox,
            state: ActionType::Walk,
            walk,
            attack,
            death,
        }
    }

    pub fn update(&mut self, velocity: V2, action: ActionType, dir: i32) {
        self.position.x += velocity.x;
        self.position.y += velocity.y;
        self.state = action;

        match self.state {  
            ActionType::Attack => {
                if self.attack.current_frame >= self.attack.limit.x {
                    self.attack.reset();
                    self.state = ActionType::Walk
                } else {
                    self.attack.update(dir);
                }
            },
            ActionType::Walk => {
                if velocity.x != 0.0 && velocity.y != 0.0 {
                     self.walk.update(dir);
                }
            },
            ActionType::Death => {
                self.death.update(dir);
            },
        }

    }
}
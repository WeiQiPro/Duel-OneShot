use macroquad::prelude::*;

use crate::{
    animation::Animation,
    graphics::Renderer,
    player::{ActionType, Player},
    utils::{Direction, V2},
};

#[derive(PartialEq)]
pub enum StateType {
    Play,
    Exit,
}

pub struct Game {
    players: [Player; 2],
    pub state: StateType,
}

impl Game {
    pub fn new(walk: Animation, attack: Animation, death: Animation) -> Self {
        let players = [
            Player::new(
                V2 {
                    x: 1280.0 / 4.0,
                    y: 360.0,
                },
                60.0,
                walk.clone(),
                attack.clone(),
                death.clone(),
            ),
            Player::new(
                V2 {
                    x: 1280.0 * 0.75,
                    y: 360.0,
                },
                60.0,
                walk.clone(),
                attack.clone(),
                death.clone(),
            ),
        ];

        Game {
            players,
            state: StateType::Play,
        }
    }

    pub async fn run(&mut self) {
        while self.state != StateType::Exit {
            // Handle events or input here (if any)

            // Update all players or entities
            for player in self.players.iter_mut() {
                player.update(
                    V2 { x: 0.0, y: 0.0 },
                    ActionType::Walk,
                    Direction::East as i32,
                );
            }

            // Render all players or entities
            for player in self.players.iter() {
                Renderer::draw_character(player);
            }

            // Await the next frame
            next_frame().await;
        }
    }
}

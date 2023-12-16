
use macroquad::{
    texture::{load_texture, Texture2D},
    time::get_time,
};

use crate::utils::V2i32;

#[derive(Clone)]
pub struct Animation {
    pub texture: Texture2D, // Sprite sheet
    pub limit: V2i32,       // Vector with the limit of frames (x, y) -> (24, 8)
    pub frames: i32,        // Number of frames
    pub duration: f32,      // Time per frame in seconds
    pub current_frame: i32, // Current frame x position
    pub direction: i32,     // Current frame y position
    pub frame_width: i32,   // Width of each frame
    pub frame_height: i32,  // Height of each frame
    pub last_update: f64,   // Last time the frame was updated
}

impl Animation {

    pub fn reset(&mut self) {
        self.current_frame = 0;
    }

    pub fn update(&mut self, dir: i32) {
        self.direction = dir;
        let now = get_time();
        if now - self.last_update >= self.duration as f64 {
            self.current_frame = (self.current_frame + 1) % self.limit.x;
            self.last_update = now;
        }
    }

    pub async fn create_walk() -> Animation {
        let walk_texture = load_texture("src/character/Walk.png").await.unwrap();
        let walk = {
            let limit: V2i32 = V2i32 { x: 24, y: 8 };

            Animation {
                texture: walk_texture,
                limit,
                frames: 24,
                duration: 0.02,
                current_frame: 0,
                direction: 0,
                frame_width: 256,
                frame_height: 256,
                last_update: get_time(),
            }
        };

        walk
    }

    pub async fn create_attack() -> Animation {
        let attack_texture = load_texture("src/character/MeleeAttack.png").await.unwrap();
        let attack = {
            let limit: V2i32 = V2i32 { x: 24, y: 8 };

            Animation {
                texture: attack_texture,
                limit,
                frames: 24,
                duration: 0.04,
                current_frame: 0,
                direction: 0,
                frame_width: 256,
                frame_height: 256,
                last_update: get_time(),
            }
        };

        attack
    }

    pub async fn create_death() -> Animation {
        let death_texture = load_texture("src/character/Death.png").await.unwrap();
        let death = {
            let limit: V2i32 = V2i32 { x: 24, y: 8 };

            Animation {
                texture: death_texture,
                limit,
                frames: 24,
                duration: 0.04,
                current_frame: 0,
                direction: 0,
                frame_width: 256,
                frame_height: 256,
                last_update: get_time(),
            }
        };
        death
    }
}

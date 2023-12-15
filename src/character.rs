use macroquad::prelude::*;

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub v: f32,
    pub walk_animation: Animation,
    pub attack_animation: Animation,
    pub is_moving: bool,
    pub is_attacking: bool,
    pub attack_radius: f32,
    pub hitbox: f32
}

pub struct Animation {
    texture: Texture2D, // Sprite sheet
    pub frames: i32, // Number of frames
    duration: f32, // Time per frame in seconds
    pub current_frame: i32, // Current frame x position
    pub direction: i32, // Current frame y position
    frame_width: i32, // Width of each frame
    frame_height: i32, // Height of each frame
    last_update: f64, // Last time the frame was updated
}

impl Animation {
    pub fn new(texture: Texture2D, frames: i32, duration: f32, direction: i32) -> Animation {
        Animation {
            texture,
            frames,
            duration,
            current_frame: 0,
            direction,
            frame_width: 256,
            frame_height: 256,
            last_update: get_time(),
        }
    }

    pub fn update(&mut self) {
        let now = get_time();
        if now - self.last_update >= self.duration as f64 {
            self.current_frame = (self.current_frame + 1) % self.frames;
            self.last_update = now;
        }
    }

    pub fn draw(&self, x: f32, y: f32) {
        let frame_x = self.current_frame * self.frame_width;
        let frame_y = self.direction * self.frame_height;
        let source_rect = Rect::new(frame_x as f32, frame_y as f32, self.frame_width as f32, self.frame_height as f32);

        draw_texture_ex(
            &self.texture,
            x,
            y,
            WHITE,
            DrawTextureParams {
                source: Some(source_rect),
                ..Default::default()
            },
        );
    }
}

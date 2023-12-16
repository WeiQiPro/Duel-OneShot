use macroquad::prelude::*;
use crate::player::{Player, ActionType};
pub(crate) struct Renderer {}

impl Renderer {
    pub fn draw_character(entity: &Player) {
        let (texture, source_rect) = match entity.state {
            ActionType::Walk => {
                let frame_x = entity.walk.current_frame * entity.walk.frame_width;
                let frame_y = entity.walk.direction * entity.walk.frame_height;
                (&entity.walk.texture, Rect::new(frame_x as f32, frame_y as f32, entity.walk.frame_width as f32, entity.walk.frame_height as f32))
            },
            ActionType::Attack => {
                let frame_x = entity.attack.current_frame * entity.attack.frame_width;
                let frame_y = entity.attack.direction * entity.attack.frame_height;
                (&entity.attack.texture, Rect::new(frame_x as f32, frame_y as f32, entity.attack.frame_width as f32, entity.attack.frame_height as f32))
            },
            ActionType::Death => {
                let frame_x = entity.death.current_frame * entity.death.frame_width;
                let frame_y = entity.death.direction * entity.death.frame_height;
                (&entity.death.texture, Rect::new(frame_x as f32, frame_y as f32, entity.death.frame_width as f32, entity.death.frame_height as f32))
            },
        };

        draw_texture_ex(
            texture,
            entity.position.x,
            entity.position.y,
            WHITE,
            DrawTextureParams {
                source: Some(source_rect),
                ..Default::default()
            },
        );
    }
}

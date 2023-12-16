use macroquad::prelude::*;
pub struct RectEntity {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub color: Color,
}

pub struct V2 {
    pub x: f32,
    pub y: f32,
}
#[derive(Clone)]
pub struct V2i32 {
    pub x: i32,
    pub y: i32,
}

impl V2 {
    pub fn new(x: f32, y: f32) -> Self {
        V2 { x, y }
    }

    pub fn normalize(&self) -> V2 {
        let norm = (self.x.powi(2) + self.y.powi(2)).sqrt();
        if norm > 0.0 {
            V2 {
                x: self.x / norm,
                y: self.y / norm,
            }
        } else {
            V2::new(0.0, 0.0)
        }
    }
}

pub enum Direction {
    East,
    North,
    NorthEast,
    NorthWest,
    South,
    SouthEast,
    SouthWest,
    West,
}
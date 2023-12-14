use macroquad::prelude::*;

pub struct RectEntity {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub color: Color,
}

pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vector2 { x, y }
    }

    pub fn normalize(&self) -> Vector2 {
        let norm = (self.x.powi(2) + self.y.powi(2)).sqrt();
        if norm > 0.0 {
            Vector2 {
                x: self.x / norm,
                y: self.y / norm,
            }
        } else {
            Vector2::new(0.0, 0.0)
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
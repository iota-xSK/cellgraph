use egui_macroquad::egui::{Pos2, Vec2};
use macroquad::prelude::Vec2 as MacroquadVec2;
use serde::{Deserialize, Serialize};
use std::ops;

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Into<Vec2> for Vector2 {
    fn into(self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.y,
        }
    }
}
impl Into<MacroquadVec2> for Vector2 {
    fn into(self) -> MacroquadVec2 {
        MacroquadVec2 {
            x: self.x,
            y: self.y,
        }
    }
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Vector2 {
        Vector2 { x, y }
    }

    pub fn zero() -> Vector2 {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl ops::Add<Vector2> for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Vector2 {
        Vector2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl ops::AddAssign<Vector2> for Vector2 {
    fn add_assign(&mut self, rhs: Vector2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::SubAssign<Vector2> for Vector2 {
    fn sub_assign(&mut self, rhs: Vector2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl ops::MulAssign<f32> for Vector2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl ops::DivAssign<f32> for Vector2 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl ops::Sub<Vector2> for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Vector2) -> Vector2 {
        Vector2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl ops::Mul<f32> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: f32) -> Vector2 {
        Vector2::new(self.x * rhs, self.y * rhs)
    }
}

impl ops::Div<f32> for Vector2 {
    type Output = Vector2;

    fn div(self, rhs: f32) -> Vector2 {
        Vector2::new(self.x / rhs, self.y / rhs)
    }
}

impl Vector2 {
    pub fn normalized(&self) -> Vector2 {
        let length = (self.x * self.x + self.y * self.y).sqrt();
        Vector2::new(self.x / length, self.y / length)
    }
}

impl Into<Pos2> for Vector2 {
    fn into(self) -> Pos2 {
        Pos2 {
            x: self.x,
            y: self.y,
        }
    }
}

impl From<(f32, f32)> for Vector2 {
    fn from(value: (f32, f32)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

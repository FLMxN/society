use crate::people::Character::{Choleric, Melancholic, Phlegmatic, Sanguine};
use macroquad::math::Vec2;
use macroquad::color::{Color, colors};

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum Character {
    Choleric,
    Melancholic,
    Sanguine,
    Phlegmatic
}

pub struct Entity {
    pub pos: Vec2,
    pub kind: Character,
    pub speed: Vec2,
    pub radius: f32
}

impl Entity {
    pub fn new(pos: Vec2, kind: Character, speed: Vec2, radius: f32) -> Self {
        Self { pos, kind, speed, radius }
    }

    pub fn color(&self) -> Color {
        match self.kind {
            Choleric => colors::RED,
            Sanguine => colors::GREEN,
            Melancholic => colors::BLUE,
            Phlegmatic => colors::WHITE,
        }
    }

    pub fn integrate(&mut self, dt: f32) {
        self.pos += self.speed * dt;
        self.speed *= 0.94;
    }
}
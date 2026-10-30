use crate::people::Character::{Choleric, Melancholic, Phlegmatic, Sanguine};
use macroquad::math::{Vec2, vec2};
use macroquad::color::{Color, colors};

// const MAX_SPEED: Vec2 = vec2(100.0, 100.0);

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
        let multiplier = match self.kind {
            Choleric => -1.01,
            Sanguine => 1.001,
            Melancholic => -0.99,
            Phlegmatic => 0.999
        };
        self.pos += self.speed * dt;
        // if (self.speed - MAX_SPEED).length() < 1e-4 {
        //     self.speed *= multiplier;
        // }
        self.speed *= multiplier;
    }
}
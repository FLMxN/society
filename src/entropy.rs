extern crate rand;

use rand::Rng;

use crate::entropy::GenType::{TINY, LARGE, LITTLE, MEDIUM, HUGE, NORMAL};

pub enum GenType {
    TINY, LITTLE, MEDIUM, LARGE, HUGE, NORMAL
}

pub fn genf32(mode: GenType) -> f32 {
    let mut rng = rand::thread_rng();
    match mode {
        TINY => rng.gen_range(0.0..1.0) as f32,
        LITTLE => rng.gen_range(1.0..5.0) as f32,
        MEDIUM => rng.gen_range(5.0..50.0) as f32,
        LARGE => rng.gen_range(50.0..500.0) as f32,
        HUGE => rng.gen_range(7.0..777.0) as f32,
        NORMAL => rng.gen_range(-5.0..5.0) as f32
    }
}
extern crate rand;

use rand::Rng;

use crate::entropy::GenType::{LARGE, LITTLE, MEDIUM};

pub enum GenType {
    LITTLE, MEDIUM, LARGE
}

pub fn genf32(mode: GenType) -> f32 {
    let mut rng = rand::thread_rng();
    match mode {
        LITTLE => rng.gen_range(1..5) as f32,
        MEDIUM => rng.gen_range(5..50) as f32,
        LARGE => rng.gen_range(50..500) as f32
    }
}
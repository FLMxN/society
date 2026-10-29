extern crate rand;

use rand::Rng;

use crate::entropy::GenType::{LARGE, LITTLE, MEDIUM};

pub enum GenType {
    LITTLE, MEDIUM, LARGE
}

pub fn genf32(mode: GenType) -> f32 {
    let mut rng = rand::thread_rng();
    match mode {
        LITTLE => rng.gen_range(0..10) as f32,
        MEDIUM => rng.gen_range(0..100) as f32,
        LARGE => rng.gen_range(0..1000) as f32
    }
}
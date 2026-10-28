mod people;
mod physics;

use macroquad::prelude::*;
extern crate rand;

use rand::Rng;

use crate::people::Entity;
use crate::people::Character::{Choleric, Sanguine, Phlegmatic, Melancholic};

use crate::physics::{flow, clash};

#[macroquad::main("society v0.1")]
async fn main() {
    let mut rng = rand::thread_rng();

    let mut population = vec![
        Entity::new(vec2(rng.gen_range(0..1000) as f32, rng.gen_range(0..1000) as f32), Choleric, vec2(1.0, 1.5), rng.gen_range(0..100) as f32),
        Entity::new(vec2(rng.gen_range(0..1000) as f32, rng.gen_range(0..1000) as f32), Choleric, vec2(1.0, 1.5), rng.gen_range(0..100) as f32),
        Entity::new(vec2(rng.gen_range(0..1000) as f32, rng.gen_range(0..1000) as f32), Choleric, vec2(1.0, 1.5), rng.gen_range(0..100) as f32),
        Entity::new(vec2(rng.gen_range(0..1000) as f32, rng.gen_range(0..1000) as f32), Choleric, vec2(1.0, 1.5), rng.gen_range(0..100) as f32),

        Entity::new(vec2(rng.gen_range(0..1000) as f32, rng.gen_range(0..1000) as f32), Sanguine, vec2(1.0, 1.5), rng.gen_range(0..100) as f32),
        Entity::new(vec2(rng.gen_range(0..1000) as f32, rng.gen_range(0..1000) as f32), Sanguine, vec2(1.0, 1.5), rng.gen_range(0..100) as f32)
    ];

    loop {
        let dt = get_frame_time().min(0.033);

        for x in &mut population {
            x.integrate(dt);
        }

        flow(&mut population, dt, 400.0);

        clash(&mut population);
        clash(&mut population);

        clear_background(BLACK);
        for x in &population {
            draw_circle(x.pos.x, x.pos.y, x.radius, x.color());
        }

        next_frame().await;
    }
}

mod people;
mod physics;
mod entropy;

use std::env;

use macroquad::input::KeyCode::L;
use macroquad::prelude::*;
extern crate rand;

use crate::people::Entity;
use crate::people::Character::{Choleric, Sanguine, Phlegmatic, Melancholic};
use crate::entropy::GenType::{LITTLE, MEDIUM, LARGE};
use crate::entropy::genf32;

use crate::physics::{flow, clash, constrain};

#[macroquad::main("society v0.2")]
async fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();
    let n= args.pop().unwrap_or(String::try_from("1").unwrap()).parse::<i32>().unwrap();
    let mut population: Vec<Entity> = vec![];
    for _ in 0..n {
        population.push(Entity::new(vec2(genf32(LARGE), genf32(LARGE)), Choleric, vec2(100.0,100.0), genf32(LITTLE)));
        population.push(Entity::new(vec2(genf32(LARGE), genf32(LARGE)), Sanguine, vec2(100.0,100.0), genf32(LITTLE)));
        population.push(Entity::new(vec2(genf32(LARGE), genf32(LARGE)), Phlegmatic, vec2(100.0,100.0), genf32(LITTLE)));
        population.push(Entity::new(vec2(genf32(LARGE), genf32(LARGE)), Melancholic, vec2(100.0,100.0), genf32(LITTLE)));
    }

    loop {
        let dt = get_frame_time().min(0.05);

        for x in &mut population {
            x.integrate(dt);
        }

        flow(&mut population, dt, 10000.0);

        clash(&mut population);
        clash(&mut population);

        constrain(&mut population, screen_width(), screen_height());

        clear_background(BLACK);
        for x in &population {
            draw_circle(x.pos.x, x.pos.y, x.radius, x.color());
        }

        next_frame().await;
    }
}

mod people;
mod physics;
mod entropy;

use std::env;

use macroquad::prelude::*;
extern crate rand;

use crate::people::Entity;
use crate::people::Character::{Choleric, Sanguine, Phlegmatic, Melancholic};
use crate::entropy::GenType::{HUGE, LARGE, LITTLE, MEDIUM, NORMAL};
use crate::entropy::genf32;

use crate::physics::{flow, clash, constrain};

#[macroquad::main("society v0.3")]
async fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();
    let n= args.pop().unwrap_or(String::try_from("0").unwrap()).parse::<i32>().unwrap();
    let mut population: Vec<Entity> = vec![];
    let mut continuous = false;
    if n > 0 {
        for _ in 0..n {
            population.push(Entity::new(vec2(genf32(HUGE), genf32(HUGE)), Choleric, vec2(genf32(NORMAL),genf32(NORMAL))*10.0, genf32(LITTLE)));
            population.push(Entity::new(vec2(genf32(HUGE), genf32(HUGE)), Sanguine, vec2(genf32(NORMAL),genf32(NORMAL))*10.0, genf32(LITTLE)));
            population.push(Entity::new(vec2(genf32(HUGE), genf32(HUGE)), Phlegmatic, vec2(genf32(NORMAL),genf32(NORMAL))*10.0, genf32(LITTLE)));
            population.push(Entity::new(vec2(genf32(HUGE), genf32(HUGE)), Melancholic, vec2(genf32(NORMAL),genf32(NORMAL))*10.0, genf32(LITTLE)));
        }
    }
    else {
        continuous = true;
    }

    let mut strength = 10000.0;
    loop {
        let dt = get_frame_time();

        if continuous && dt % 2.0 < 0.02 {
            population.push(Entity::new(vec2(genf32(HUGE), genf32(HUGE)), Choleric, vec2(genf32(NORMAL),genf32(NORMAL))*10.0, genf32(LITTLE)));
            population.push(Entity::new(vec2(genf32(HUGE), genf32(HUGE)), Sanguine, vec2(genf32(NORMAL),genf32(NORMAL))*10.0, genf32(LITTLE)));
            population.push(Entity::new(vec2(genf32(HUGE), genf32(HUGE)), Phlegmatic, vec2(genf32(NORMAL),genf32(NORMAL))*10.0, genf32(LITTLE)));
            population.push(Entity::new(vec2(genf32(HUGE), genf32(HUGE)), Melancholic, vec2(genf32(NORMAL),genf32(NORMAL))*10.0, genf32(LITTLE)));
        }

        for x in &mut population {
            x.integrate(dt);
        }

        flow(&mut population, dt, strength);

        clash(&mut population);
        clash(&mut population);

        constrain(&mut population, screen_width(), screen_height());

        clear_background(BLACK);
        for x in &population {
            draw_circle(x.pos.x, x.pos.y, x.radius, x.color());
        }
        strength -= 1.0;
        next_frame().await;
    }
}

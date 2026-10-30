use std::collections::HashMap;

use macroquad::math::Vec2;

use crate::people::{Character, Entity};

pub fn flow(xs: &mut [Entity], dt: f32, strength: f32) {
    let mut sums: HashMap<Character, (Vec2, f32)> = HashMap::new();
    for x in xs.iter() {
        let entry = sums.entry(x.kind).or_insert((Vec2::ZERO, 0.0));
        entry.0 += x.pos;
        entry.1 += 1 as f32;
    }

    for x in xs.iter_mut() {
        if let Some(&(sum, count)) = sums.get(&x.kind) {
            if count <= 1.0 { continue; }
            let centroid = sum / count as f32;
            let dir = (centroid - x.pos).normalize_or_zero();
            x.speed += dir * strength * dt;
        }
    }
}

pub fn clash(xs: &mut [Entity]) {
    for i in 0..xs.len() {
        for j in (i + 1)..xs.len() {
            // if xs[i].kind != xs[j].kind {
            //     continue;
            // }

            let delta = xs[j].pos - xs[i].pos;
            let dist = delta.length();
            let min_dist = xs[i].radius + xs[j].radius;
            if dist >= min_dist || dist < 1e-4 {
                continue;
            }

            let overlap = min_dist - dist;
            let n = delta / dist;

            xs[i].pos -= n * overlap * 0.5;
            xs[j].pos += n * overlap * 0.5;

            let rel = xs[j].speed - xs[i].speed;
            let along = rel.dot(n);
            if along < 0.0 {
                let impulse = n * along * 0.5;
                xs[i].speed += impulse;
                xs[j].speed -= impulse;
            }
        }
    }
}

pub fn constrain(population: &mut [Entity], width: f32, height: f32) {
    for x in population.iter_mut() {
        if x.pos.x - x.radius < 0.0 {
            x.pos.x = x.radius;
            if x.speed.x < 0.0 {
                x.speed.x = -x.speed.x;
            }
        }
        if x.pos.x + x.radius > width {
            x.pos.x = width - x.radius;
            if x.speed.x > 0.0 {
                x.speed.x = -x.speed.x;
            }
        }
        if x.pos.y - x.radius < 0.0 {
            x.pos.y = x.radius;
            if x.speed.y < 0.0 {
                x.speed.y = -x.speed.y;
            }
        }
        if x.pos.y + x.radius > height {
            x.pos.y = height - x.radius;
            if x.speed.y > 0.0 {
                x.speed.y = -x.speed.y;
            }
        }
    }
}
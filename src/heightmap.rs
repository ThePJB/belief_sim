use minvect::*;
use crate::noise::*;
use std::f32::{INFINITY, NEG_INFINITY};

fn gradient(cols: &[(f32, Vec4)], t: f32) -> Vec4 {
    for i in 1..cols.len() {
        let lo = cols[i-1].0;
        let hi = cols[i].0;

        if t > lo && t <= hi {
            let range = hi-lo;
            let t = (t - lo) / range;
            return cols[i-1].1.lerp(cols[i].1, t);
        }
    }
    dbg!(cols);
    dbg!(t);
    panic!("bad gradient");
}

pub fn col(p: &Vec2, seed: u32) -> Vec4 {
    let col_deep_water = vec4(0.0, 0.3, 0.6, 1.0);
    let col_shallow_water = vec4(0.0, 0.6, 0.8, 1.0);
    let col_plains = vec4(0.4, 0.8, 0.4, 1.0);
    let col_beach = vec4(0.8, 0.8, 0.3, 1.0);
    let col_mountain = vec4(0.5, 0.5, 0.5, 1.0);
    let col_snow = vec4(1.0, 1.0, 1.0, 1.0);
    let col_forest = vec4(0.1, 0.5, 0.1, 1.0);

    let h_gradient = vec![
        (NEG_INFINITY, col_deep_water),
        (0.0, col_deep_water),
        (0.6, col_shallow_water),
        (0.601, col_beach),
        (0.68, col_plains),
        (1.4, col_forest),
        (1.5, col_mountain),
        (2.0, col_snow),
        (INFINITY, col_snow),
    ];
    
    let h = heightmap(p, seed);

    let col_h = gradient(&h_gradient, h);
    col_h
}

pub fn heightmap(p: &Vec2, seed: u32) -> f32 {
    noise_exp(&(*p * 5.0), seed)
}

pub fn temperature_map(p: &Vec2, seed: u32) -> f32 {
    let h = heightmap(p, seed);
    let temp_seed = hash(seed);
    noise_grad(&(*p * 4.0), temp_seed) - h * 0.1
}
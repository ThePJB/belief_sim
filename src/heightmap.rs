use minvect::*;
use crate::noise::*;

pub fn heightmap(p: Vec2, seed: u32) -> f32 {
    2.0*(noise_grad(&(p * 4.0), seed) + 0.5)
}

pub fn temperature_map(p: Vec2, seed: u32) -> f32 {
    let h = heightmap(p, seed);
    let temp_seed = hash(seed);
    noise_grad(&(p * 4.0), temp_seed) - h * 0.1
}
use std::f32::consts::PI;
use minvect::*;

pub fn hash(mut state: u32) -> u32 {
    state = (state ^ 2747636419).wrapping_mul(2654435769);
    state = (state ^ (state >> 16)).wrapping_mul(2654435769);
    state = (state ^ (state >> 16)).wrapping_mul(2654435769);
    state
}

pub fn rand(seed: u32) -> f32 {
    hash(seed) as f32 / 4294967295.0
}

pub fn lerp(x: f32, y: f32, t: f32) -> f32 {
    x * (1.0 - t) + y * t
}

pub fn fade(t: f32) -> f32 {
    t*t*t*(t*(t*6.0-15.0)+10.0)
}

// can provide analytical gradient pretty easily
// can you influence the vectors and stuff in octaves?
// possibility of hyper optimizations
// eg u64 hash, or optimization of rng bits. dont need as many, like if we were selcting from 8directions. can also do shitty 0-1 0-1 and normalize unit vectors. or not bother
pub fn noise_grad(p: &Vec2, seed: u32) -> f32 {
    // grid corners get a random vector
    // blend the vectors
    let i = vec2(1.0, 0.0);
    let j = vec2(0.0, 1.0);

    let p00 = vec2(p.x.floor(), p.y.floor());
    let p01 = p00 + j;
    let p10 = p00 + i;
    let p11 = p00 + i + j;

    let a00 = 2.0*PI*rand(seed.wrapping_add(1512347u32.wrapping_mul(p00.x as u32).wrapping_add(213154127u32.wrapping_mul(p00.y as u32))));
    let a01 = 2.0*PI*rand(seed.wrapping_add(1512347u32.wrapping_mul(p01.x as u32).wrapping_add(213154127u32.wrapping_mul(p01.y as u32))));
    let a10 = 2.0*PI*rand(seed.wrapping_add(1512347u32.wrapping_mul(p10.x as u32).wrapping_add(213154127u32.wrapping_mul(p10.y as u32))));
    let a11 = 2.0*PI*rand(seed.wrapping_add(1512347u32.wrapping_mul(p11.x as u32).wrapping_add(213154127u32.wrapping_mul(p11.y as u32))));

    let u00 = vec2(a00.cos(), a00.sin());
    let u01 = vec2(a01.cos(), a01.sin());
    let u10 = vec2(a10.cos(), a10.sin());
    let u11 = vec2(a11.cos(), a11.sin());

    // instead of unif rand. can do exp here
    let m00 = rand(seed.wrapping_add(467897247u32.wrapping_mul(p00.x as u32).wrapping_add(3195781247u32.wrapping_mul(p00.y as u32))));
    let m01 = rand(seed.wrapping_add(467897247u32.wrapping_mul(p01.x as u32).wrapping_add(3195781247u32.wrapping_mul(p01.y as u32))));
    let m10 = rand(seed.wrapping_add(467897247u32.wrapping_mul(p10.x as u32).wrapping_add(3195781247u32.wrapping_mul(p10.y as u32))));
    let m11 = rand(seed.wrapping_add(467897247u32.wrapping_mul(p11.x as u32).wrapping_add(3195781247u32.wrapping_mul(p11.y as u32))));

    let s00 = (*p - p00).dot(m00*u00);
    let s01 = (*p - p01).dot(m01*u01);
    let s10 = (*p - p10).dot(m10*u10);
    let s11 = (*p - p11).dot(m11*u11);

    let fx = p.x.fract();
    let fy = p.y.fract();

    let tx = fade(fx);
    let ty = fade(fy);

    let ly0 = lerp(s00, s01, ty);
    let ly1 = lerp(s10, s11, ty);
    let l = lerp(ly0, ly1, tx);

    return l;
}
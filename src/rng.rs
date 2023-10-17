use std::time::SystemTime;

pub fn khash(mut state: u32) -> u32 {
    state = (state ^ 2747636419).wrapping_mul(2654435769);
    state = (state ^ (state >> 16)).wrapping_mul(2654435769);
    state = (state ^ (state >> 16)).wrapping_mul(2654435769);
    state
}
pub fn krand(seed: u32) -> f32 {
    khash(seed) as f32 / 4294967295.0
}

pub fn random_seed() -> u32 {
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).map(|x| x.as_nanos() as u32).unwrap_or(1)
}

#[derive(Clone)]
pub struct Rng {
    seed: u32,
}

impl Rng {
    pub fn new_random() -> Self {
        Rng { seed:  random_seed() }
    }
    pub fn new_seeded(starting_seed: u32) -> Self {
        Rng { seed: starting_seed }
    }
    pub fn next_float(&mut self) -> f32 {
        self.seed = khash(self.seed) + 69;
        self.seed as f32 / 4294967295.0
    }
    pub fn next_u32(&mut self) -> u32 {
        self.seed = khash(self.seed) + 69;
        khash(self.seed)
        
    }
    pub fn next_u64(&mut self) -> u64 {
        (self.next_u32() as u64) << 16 | self.next_u32() as u64
    }
}
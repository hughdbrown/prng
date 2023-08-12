use rand::prelude::*;

// Return a pseudorandom value in the range [min, max).

pub struct Prng {
    prng: ThreadRng,
}

impl Prng {
    pub fn new() -> Self {
        let prng = thread_rng();
        Prng { prng }
    }

    pub fn next_u64(mut self, min: u64, max: u64) -> u64 {
        let range = (max - min) as u64;
        min + self.prng.next_u64() % range
    }
}

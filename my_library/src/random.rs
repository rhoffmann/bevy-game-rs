#[cfg(feature = "xorshift")]
type RngCore = rand_xorshift::XorShiftRng;

#[cfg(feature = "pcg")]
type RngCore = rand_pcg::Pcg64Mcg;

#[cfg(all(not(feature = "xorshift"), not(feature = "pcg")))]
type RngCore = rand::prelude::StdRng;

use rand::{
    Rng, SeedableRng,
    distr::{
        Distribution, StandardUniform,
        uniform::{SampleRange, SampleUniform},
    },
};

pub struct RandomNumberGenerator {
    rng: RngCore,
}

impl RandomNumberGenerator {
    pub fn new() -> Self {
        Self {
            rng: RngCore::from_os_rng(),
        }
    }
    pub fn seeded(seed: u64) -> Self {
        Self {
            rng: RngCore::seed_from_u64(seed),
        }
    }

    pub fn range<T>(&mut self, range: impl SampleRange<T>) -> T
    where
        T: SampleUniform + PartialOrd,
    {
        self.rng.random_range(range)
    }

    pub fn next<T>(&mut self) -> T
    where
        StandardUniform: Distribution<T>,
    {
        self.rng.random()
    }
}

impl Default for RandomNumberGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_range_bounds() {
        let mut rng = RandomNumberGenerator::new();
        for _ in 0..1000 {
            let n = rng.range(1..=10);
            assert!(n >= 1);
            assert!(n <= 10);
        }
    }

    #[test]
    fn test_range_floats() {
        let mut rng = RandomNumberGenerator::new();
        for _ in 0..1000 {
            let f = rng.range(-5000.0f32..5000.0f32);
            assert!(f >= -5000.0);
            assert!(f < 5000.0);
        }
    }

    #[test]
    fn test_reproducibility() {
        let mut rng = (
            RandomNumberGenerator::seeded(1),
            RandomNumberGenerator::seeded(1),
        );
        (0..1000).for_each(|_| {
            assert_eq!(
                rng.0.range(u32::MIN..u32::MAX),
                rng.1.range(u32::MIN..u32::MAX)
            )
        })
    }

    #[test]
    fn test_next_generic() {
        let mut rng = RandomNumberGenerator::new();
        let n: u8 = rng.next();
        assert!(n <= u8::MAX);
        let f: f64 = rng.next();
        assert!(f >= 0.0 && f < 1.0);
        let g = rng.next::<f32>();
        assert!(g >= 0.0 && g < 1.0);
    }
}

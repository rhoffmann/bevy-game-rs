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

/// `RandomNumberGenerator` holds random number generation state and offers random number generation services.
///
/// `RandomNumberGnerator` defaults to using the [PCG](https://crates.io/crates/rand_pcg) algorithm.
/// You can specify an [XorShift](https://crates.io/crates/rand_xorshift) algorithm by enabling the `xorshift` feature flags when including `my_library` as a dependency.
///
/// By default, `RandomNumberGenerator` requires mutability --- it is shared in Bevy applications as `ResMut<RandomNumberGenerator>`.
/// If you want to use it in a multi-threaded context as `Res<RandomNumberGenerator>`, enable the `locking` feature flag to use the interior-mutable version.
///
/// ## Example
///
/// ```
/// use my_library::RandomNumberGenerator;
/// let mut rng = RandomNumberGenerator::new();
/// let n: u32 = rng.range(1..=10);
/// println!("Random number between 1 and 10: {}", n);
/// ```
#[derive(bevy::prelude::Resource)]
pub struct RandomNumberGenerator {
    rng: RngCore,
}

impl RandomNumberGenerator {
    /// Create a new RandomNumberGenerator seeded from a os starting seed
    pub fn new() -> Self {
        Self {
            rng: RngCore::from_os_rng(),
        }
    }
    /// Create a new RandomNumberGenerator seeded from the given seed .
    /// It will produce the same sequence of random numbers for the same seed.
    ///
    /// # Arguments
    ///
    /// * `seed` - A u64 seed to initialize the random number generator.
    ///
    /// # Example
    ///
    /// ```
    /// use my_library::RandomNumberGenerator;
    /// let mut rng1 = RandomNumberGenerator::seeded(42);
    /// let mut rng2 = RandomNumberGenerator::seeded(42);
    /// assert_eq!(rng1.range(1..=100), rng2.range(1..=100));
    /// ```
    pub fn seeded(seed: u64) -> Self {
        Self {
            rng: RngCore::seed_from_u64(seed),
        }
    }

    /// Gnerate a random value within the specified range.
    ///
    /// # Arguments
    ///
    /// * `range` - A range specifying the bounds for the random value.
    ///
    /// # Example
    ///
    /// ```
    /// use my_library::RandomNumberGenerator;
    /// let mut rng = RandomNumberGenerator::new();
    /// let one_to_nine: u32 = rng.range(1..10);
    /// let one_to_ten=: u32 = rng.range(1..=10);
    pub fn range<T>(&mut self, range: impl SampleRange<T>) -> T
    where
        T: SampleUniform + PartialOrd,
    {
        self.rng.random_range(range)
    }

    /// Generate the next random value of the requested type.
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

/// `RandomPlugin` is a Bevy plugin that adds `RandomNumberGenerator` as a resource.
///
/// Once you att the plugin (with `App::new().add_plugin(RandomPlugin)`), you can access the RNG in your systems as `ResMut<RandomNumberGenerator>`.
pub struct RandomPlugin;

impl bevy::prelude::Plugin for RandomPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(RandomNumberGenerator::new());
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

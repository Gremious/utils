use super::*;

/// Debug rng so you can just do `rng(0.0..1.0)` or `rng(1..100)` or whatever no effort
pub fn rng<T: PartialOrd + rand::distributions::uniform::SampleUniform>(range: core::ops::Range<T>) -> T {
	use rand::Rng;
    rand::thread_rng().gen_range(range)
}

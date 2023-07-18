pub trait Normalize<N> {
	fn normalize(&mut self, range: std::ops::Range<N>);
}

impl<N: num_traits::Float> Normalize<N> for Vec<N> {
	fn normalize(&mut self, range: std::ops::Range<N>) {
		let max = self.iter().max_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal)).copied().unwrap_or(N::epsilon());
		let min = self.iter().min_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal)).copied().unwrap_or(N::epsilon());

		self.iter_mut().for_each(|x| *x = (range.end - range.start) * ((*x - min) / (max - min)) + range.start);
	}
}

#[test]
fn test_normalize() {
	use crate::common_prelude::*;

	assert_eq!(vec![1., 2., 3.].tap_mut(|x| x.normalize(0.0..1.0)), vec![0., 0.5, 1.]);
	assert_eq!(vec![1., 2., 3.].tap_mut(|x| x.normalize(-1.0..1.0)), vec![-1., 0., 1.]);
}

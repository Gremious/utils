pub trait Normalize {
	fn normalize(&mut self, range: std::ops::Range<f32>);
}

impl Normalize for Vec<f32> {
	fn normalize(&mut self, range: std::ops::Range<f32>) {
		let max = self.iter().max_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal)).copied().unwrap_or(f32::EPSILON);
		let min = self.iter().min_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal)).copied().unwrap_or(f32::EPSILON);

		self.iter_mut().for_each(|x| *x = (range.end - range.start) * ((*x - min) / (max - min)) + range.start);
	}
}

#[test]
fn test_normalize() {
	use crate::common_prelude::*;

	assert_eq!(vec![1., 2., 3.].tap_mut(|x| x.normalize(0.0..1.0)), vec![0., 0.5, 1.]);
	assert_eq!(vec![1., 2., 3.].tap_mut(|x| x.normalize(-1.0..1.0)), vec![-1., 0., 1.]);
}

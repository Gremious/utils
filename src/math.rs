pub trait Normalize {
	fn normalize(&mut self, range: std::ops::Range<f32>);
}

impl Normalize for Vec<f32> {
	fn normalize(&mut self, range: std::ops::Range<f32>) {
		let max = self.iter().max_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal)).copied().unwrap_or(f32::EPSILON);
		let min = self.iter().min_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal)).copied().unwrap_or(f32::EPSILON);

		// linear transformation
        for x in self.iter_mut() { *x = range.end - range.start * ((*x - min) / (max - min)) + range.start; }
	}
}

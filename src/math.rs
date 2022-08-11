// Maye could e a goood trait for Vec<T> or something?
pub fn normalize(data: &mut [f32], range: std::ops::Range<f32>) {
	let max = data.iter().max_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal)).copied().unwrap_or(f32::EPSILON);
	let min = data.iter().min_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal)).copied().unwrap_or(f32::EPSILON);

	// linear transformation
	data.iter_mut().for_each(|x| { *x = range.end - range.start * ((*x - min) / (max - min)) + range.start; });
}


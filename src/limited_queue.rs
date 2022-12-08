use std::collections::VecDeque;

/// A queue that will never go above capacity, dropping the last element if there's an attempt to push past size.
///
/// Basically a fake of a circular ring buffer with push_front capabilities.
#[derive(Debug, shrinkwraprs::Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct LimitedQueue<const QUEUE_SIZE: usize, T>(pub VecDeque<T>);

impl<const QUEUE_SIZE: usize, T> LimitedQueue<QUEUE_SIZE, T> {
	#[must_use]
	/// Pre-populates the queue with the default value
	pub fn new_with_default() -> Self where T: Copy + Default {
		let mut queue = VecDeque::with_capacity(QUEUE_SIZE);
		queue.extend([T::default(); QUEUE_SIZE]);
		Self(queue)
	}

	pub fn push_front(&mut self, value: T) {
		if QUEUE_SIZE == self.len() { self.truncate(QUEUE_SIZE - 1); }
		self.0.push_front(value);
	}
}

impl<const QUEUE_SIZE: usize, T> Default for LimitedQueue<QUEUE_SIZE, T> {
	fn default() -> Self {
		Self(VecDeque::with_capacity(QUEUE_SIZE))
	}
}

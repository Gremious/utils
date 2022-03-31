pub trait LogIfError {
	fn log_if_err(self) -> Self;
}

impl<T, E: std::fmt::Debug> LogIfError for anyhow::Result<T, E> {
	fn log_if_err(self) -> Self {
		match &self {
			Ok(_) => {},
			Err(e) => log::error!("Error: {:?}", e),
		}
		self
	}
}

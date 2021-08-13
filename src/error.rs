pub trait LogIfError {
	fn log_err(self) -> Self;
}

impl<T> LogIfError for anyhow::Result<T> {
	fn log_err(self) -> Self {
		match &self {
			Ok(_) => {},
			Err(e) => log::error!("Error: {:?}", e),
		}
		self
	}
}

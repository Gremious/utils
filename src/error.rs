pub trait LogIfError {
	fn log_if_err(self) -> Self;
	fn log_if_err_with_msg(self, msg: &str) -> Self;
}

impl<T, E: std::fmt::Debug> LogIfError for anyhow::Result<T, E> {
	fn log_if_err(self) -> Self {
		match &self {
			Ok(_) => {},
			Err(e) => log::error!("Error: {:?}", e),
		}
		self
	}

	fn log_if_err_with_msg(self, msg: &str) -> Self {
		match &self {
			Ok(_) => {},
			Err(e) => log::error!("{}: {:?}", msg, e),
		}
		self
	}
}

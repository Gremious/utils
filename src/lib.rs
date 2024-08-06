#![feature(proc_macro_hygiene, stmt_expr_attributes)]
#![allow(async_fn_in_trait)]

pub mod serde_utils;
pub mod common_prelude;
pub mod duration;
pub mod logger;
pub mod math;
pub mod hhmmss;

pub use duration::Duration;
use common_prelude::*;

pub static REQWEST_CLIENT: Lazy<reqwest::Client> = Lazy::new(reqwest::Client::new);

#[cfg(not(target_arch = "wasm32"))]
#[track_caller]
pub fn spawn_complain_send<T>(x: impl std::future::Future<Output = anyhow::Result<T>> + Send + 'static) {
	let caller = core::panic::Location::caller();
	tokio::spawn(async move { if let Err(e) = x.await {
		let lvl = log::Level::Error;
		if lvl <= log::STATIC_MAX_LEVEL && lvl <= log::max_level() {
			log::__private_api::log(
				log::__private_api::format_args!("{e:?}"),
				lvl,
				&(log::__private_api::module_path!(), log::__private_api::module_path!(), caller),
				(),
			);
		}
	} });
}

#[must_use]
pub fn default<T: Default>() -> T { T::default() }

#[macro_export]
macro_rules! spawn_complain {
	($body: block) => { spawn_complain(async move { $body; Ok(()) }) };
}

#[cfg(target_arch = "wasm32")]
pub fn debugger() {
	web_sys::js_sys::eval("debugger").ok();
}

pub trait VerboseErrorForStatus {
	/// Basically
	///
	/// req
	///   .error_for_status()?
	///   .json::<T>().await
	///
	/// Except it will log not just the status code,
	/// but the entire json responce on error.
	/// It will also tell you which field in which sturct is missing if serde failed.
	async fn try_json<T: for<'a> serde::Deserialize<'a>>(self) -> anyhow::Result<T>;

	/// error_for_status() but it will log the json responce as well.
	///
	/// Separate trait fn for when you don't need the responce e.g. POST requests.
	async fn body_for_status(self) -> anyhow::Result<()>;
}

impl VerboseErrorForStatus for reqwest::Response {
	async fn try_json<T: for <'a> serde::Deserialize<'a>>(self) -> anyhow::Result<T> {
		let status = self.status();
		let raw_json = self.json::<serde_json::Value>().await
			.context("Got non-json responce?\nTry .text() instead of .json() and see what you get.")?;
		let type_name = std::any::type_name::<T>();

		if status.is_success() {
			// Could do "to_string_pretty" but that can fail with a map with non string keys.
			// Format is guaranteed.
			let responce_fmt = format!("{raw_json:#?}");
			let try_json = serde_json::from_value::<T>(raw_json);

			Ok(try_json.map_err(anyhow::Error::from)
				.with_context(|| format!("\nFailed to deserialize {type_name};\n\nResponce: {responce_fmt}"))?)
		} else {
			let error = format!("Status: {}: {:?}", status.as_str(), status.canonical_reason());
			Err(anyhow::anyhow!("{error}: \n{raw_json:#?}"))
		}
	}

	async fn body_for_status(self) -> anyhow::Result<()> {
		let status = self.status();

		if status.is_success() {
			Ok(())
		} else {
			let error = format!("Status: {}: {:?}", status.as_str(), status.canonical_reason());
			let json = self.json::<serde_json::Value>().await?;
			Err(anyhow::anyhow!("{error}: \n{json:#?}"))
		}
	}
}

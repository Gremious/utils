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
	/// but the entire json response on error.
	/// It will also tell you which field in which sturct is missing if serde failed.
	async fn try_json<T: for<'a> serde::Deserialize<'a>>(self) -> anyhow::Result<T>;

	/// req
	///   .error_for_status()?
	///   .json::<serde_json::Value>().await
	///
	///   So that you don't do serde_json::from_value::<serde_json::Value>()
	///   and skips the serde errors that aren't relevant
	async fn try_json_value(self) -> anyhow::Result<serde_json::Value>;

	/// error_for_status() but it will log the json response as well.
	///
	/// Separate fn for when you don't need the response e.g. some POST requests.
	async fn error_for_status_with_body(self) -> anyhow::Result<reqwest::Response>;
}

impl VerboseErrorForStatus for reqwest::Response {
	async fn try_json<T: for <'a> serde::Deserialize<'a>>(self) -> anyhow::Result<T> {
		let status = self.status();
		let raw_json = self.json::<serde_json::Value>().await
			.context("Got non-json response?\nTry .text() instead of .json() and see what you get.")?;
		let type_name = std::any::type_name::<T>();

		if status.is_success() {
			// Can't fail since raw_json succeeded already.
			let response_fmt = serde_json::to_string_pretty(&raw_json).unwrap();
			let try_json = serde_json::from_value::<T>(raw_json);

			Ok(try_json.map_err(anyhow::Error::from)
				.with_context(|| format!("\nFailed to deserialize {type_name};\n\nResponse: {response_fmt}"))?)
		} else {
			let error = format!("Status: {}: {:?}", status.as_str(), status.canonical_reason());
			Err(anyhow::anyhow!("{error}: \n{raw_json:#?}"))
		}
	}

	async fn try_json_value(self) -> anyhow::Result<serde_json::Value> {
		let status = self.status();
		let raw_json = self.json::<serde_json::Value>().await
			.context("Got non-json response?\nTry .text() instead of .json() and see what you get.")?;
		let response_fmt = serde_json::to_string_pretty(&raw_json).unwrap();

		if status.is_success() {
			Ok(raw_json)
		} else {
			let error = format!("Status: {}: {:?}", status.as_str(), status.canonical_reason());
			Err(anyhow::anyhow!("{error}: \n{response_fmt}"))
		}
	}

	async fn error_for_status_with_body(self) -> anyhow::Result<reqwest::Response> {
		let status = self.status();

		if status.is_success() {
			Ok(self)
		} else {
			let error = format!("Status: {}: {:?}", status.as_str(), status.canonical_reason());
			let response = self.text().await?;
			Err(anyhow::anyhow!("{error}: \n{response:#?}"))
		}
	}
}

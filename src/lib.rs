#![feature(proc_macro_hygiene, stmt_expr_attributes)]
#![allow(async_fn_in_trait)]

pub mod serde_utils;
pub mod common_prelude;
pub mod duration;
pub mod logger;
pub mod math;
pub mod hhmmss;
pub mod chrono_utils;

pub use duration::Duration;
use common_prelude::*;

pub static REQWEST_CLIENT: Lazy<reqwest::Client> = Lazy::new(reqwest::Client::new);

pub struct AbortOnDrop<T>(tokio::task::JoinHandle<T>);
impl<T> Drop for AbortOnDrop<T> { fn drop(&mut self) { self.0.abort(); } }

#[extend::ext(pub, name = JoinHandleExt)]
impl<T> tokio::task::JoinHandle<T> {
	fn abort_on_drop(self) -> AbortOnDrop<T> {
		AbortOnDrop(self)
	}
}

#[extend::ext(pub)]
impl bool {
	fn flip(&mut self) { *self ^= true; }
}

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

#[extend::ext(pub, name = VerboseErrorForStatus)]
impl reqwest::Response {
	/// Basically
	///
	/// req
	///   .error_for_status()?
	///   .json::<T>().await
	///
	/// Except it will log not just the status code,
	/// but the entire json response on error.
	/// It will also tell you which field in which sturct is missing if serde failed.
	async fn try_json<T: for <'a> serde::Deserialize<'a> + 'static>(self) -> anyhow::Result<T> {
		let status = self.status();
		let bytes = self.bytes().await?;
		let type_name = std::any::type_name::<T>();

		if !status.is_success() {
			if let Ok(json) = serde_json::from_slice::<serde_json::Value>(&bytes) {
				anyhow::bail!("Status: {status}: {canonical:?}:\n{json}",
					status = status.as_str(),
					canonical = status.canonical_reason(),
					json = serde_json::to_string_pretty(&json).unwrap(),
				)
			} else if let Ok(text) = std::str::from_utf8(&bytes) {
				anyhow::bail!("Status: {status}: {canonical:?}:\n{text}",
					status = status.as_str(),
					canonical = status.canonical_reason(),
				)
			} else {
				anyhow::bail!("Status: {status}: {canonical:?}: <binary>",
					status = status.as_str(),
					canonical = status.canonical_reason(),
				)
			}
		}

		let json = match serde_json::from_slice::<serde_json::Value>(&bytes) {
			Ok(json) => json,
			Err(e) => match std::str::from_utf8(&bytes) {
				Ok(text) => anyhow::bail!("Failed to parse json as {type_name}: {e}\n{text}"),
				Err(_) => anyhow::bail!("Failed to parse json as {type_name}: {e}\n<binary>"),
			}
		};
		if std::any::TypeId::of::<T>() == std::any::TypeId::of::<serde_json::Value>() {
			let res = unsafe { std::mem::transmute_copy::<serde_json::Value, T>(&json) };
			std::mem::forget(json);
			Ok(res)
		} else {
			match serde_json::from_value(json.clone()) {
				Ok(t) => Ok(t),
				Err(e) => anyhow::bail!("Failed to parse json as {type_name}: {e}\n{json}",
					type_name = type_name,
					e = e,
					json = serde_json::to_string_pretty(&json).unwrap(),
				),
			}
		}
	}

	/// error_for_status() but it will log the json response as well.
	///
	/// Separate fn for when you don't need the response e.g. some POST requests.
	async fn error_for_status_with_body(self) -> anyhow::Result<reqwest::Response> {
		let status = self.status();

		anyhow::ensure!(status.is_success(),
			"Status: {status}: {canonical:?}:\n{body}",
			status = status.as_str(),
			canonical = status.canonical_reason(),
			body = if let Ok(body) = std::str::from_utf8(&self.bytes().await?) { body.to_owned() } else { "<binary>".to_owned() },
		);

		Ok(self)
	}
}

#[cfg(not(target_arch = "wasm32"))]
#[tokio::test]
async fn try_json() {
	#[derive(serde::Deserialize)]
	struct Foo {
		a: i32,
		b: String,
		c: bool,
	}

	#[derive(serde::Deserialize)]
	struct BadFoo {
		d: String,
	}

	let mut server = mockito::Server::new_async().await;
	let host = server.host_with_port();
	let url = server.url();

	server.mock("GET", "/some-text")
		.with_status(200)
		.with_header("content-type", "text/plain")
		.with_body(r#"haha this not json dummy"#)
		.create_async().await;
	server.mock("GET", "/some-json")
		.with_status(200)
		.with_header("content-type", "application/json")
		.with_body(r#"{"a":1}"#)
		.create_async().await;
	server.mock("GET", "/foo-json")
		.with_status(200)
		.with_header("content-type", "application/json")
		.with_body(r#"{"a":1,"b":"ahha","c":true}"#)
		.create_async().await;
	server.mock("GET", "/400-with-text")
		.with_status(400)
		.with_header("content-type", "text/plain")
		.with_body(r#"error text"#)
		.create_async().await;
	server.mock("GET", "/400-with-json")
		.with_status(400)
		.with_header("content-type", "application/json")
		.with_body(r#"{"error":"error text"}"#)
		.create_async().await;

	assert!(reqwest::get(&format!("{url}/some-text")).await.unwrap()
		.try_json::<serde_json::Value>().await.is_err());
	assert!(reqwest::get(&format!("{url}/some-text")).await.unwrap()
		.try_json::<Foo>().await.is_err());
	assert!(reqwest::get(&format!("{url}/some-json")).await.unwrap()
		.try_json::<serde_json::Value>().await.is_ok());
	assert!(reqwest::get(&format!("{url}/foo-json")).await.unwrap()
		.try_json::<Foo>().await.is_ok());
	assert!(reqwest::get(&format!("{url}/foo-json")).await.unwrap()
		.try_json::<BadFoo>().await.is_err());
	assert!(reqwest::get(&format!("{url}/400-with-text")).await.unwrap()
		.try_json::<Foo>().await.is_err());
	assert!(reqwest::get(&format!("{url}/400-with-json")).await.unwrap()
		.try_json::<Foo>().await.is_err());
}

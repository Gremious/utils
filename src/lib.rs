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

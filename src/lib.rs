#![feature(async_fn_in_trait)]

#[cfg(feature = "hobo_plus")] pub mod hobo_plus;
pub mod auth;
pub mod serde_utils;
pub mod common_prelude;
pub mod rkyv_shims;
pub mod duration;
pub mod logger;
pub mod math;
pub mod hhmmss;

pub use duration::Duration;

use once_cell::sync::Lazy;
use serde::{Serialize, Deserialize};

pub static REQWEST_CLIENT: Lazy<reqwest::Client> = Lazy::new(reqwest::Client::new);

#[cfg(not(target_arch = "wasm32"))]
#[track_caller]
pub fn spawn_complain_send<T>(x: impl std::future::Future<Output = anyhow::Result<T>> + Send + 'static) {
	let caller = core::panic::Location::caller();
	tokio::spawn(async move { if let Err(e) = x.await {
		let lvl = log::Level::Error;
		if lvl <= log::STATIC_MAX_LEVEL && lvl <= log::max_level() {
			log::__private_api_log(
				log::__log_format_args!("{:?}", e),
				lvl,
				&(log::__log_module_path!(), log::__log_module_path!(), caller.file(), caller.line()),
				log::__private_api::Option::None,
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

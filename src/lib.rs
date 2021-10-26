#[cfg(target_arch = "wasm32")] pub mod hobo_plus;
pub mod error;
pub mod serde_utils;

use once_cell::sync::Lazy;

pub static REQWEST_CLIENT: Lazy<reqwest::Client> = Lazy::new(reqwest::Client::new);

#[cfg(target_arch = "wasm32")]
pub fn spawn_complain<T>(x: impl std::future::Future<Output = anyhow::Result<T>> + 'static) {
	wasm_bindgen_futures::spawn_local(async move { if let Err(e) = x.await { log::error!("{:?}", e); } });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn spawn_complain<T>(x: impl std::future::Future<Output = anyhow::Result<T>> + Send + 'static) {
	tokio::spawn(async move { if let Err(e) = x.await { log::error!("{:?}", e); } });
}

#[must_use]
pub fn default<T: Default>() -> T { T::default() }

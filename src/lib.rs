#[cfg(target_arch = "wasm32")] pub mod hobo_plus;
pub mod error;
pub mod serde_utils;

use once_cell::sync::Lazy;
use serde::{Serialize, Deserialize};

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

#[derive(Debug, Serialize, Deserialize, Clone, smart_default::SmartDefault, PartialEq, Eq, Hash)]
pub struct OauthToken {
	pub expires_in: i64,
	pub access_token: String,
	#[default(chrono::Utc::now())]
	#[serde(default = "chrono::Utc::now")]
	pub created_at: chrono::DateTime<chrono::Utc>,
}

impl OauthToken {
	pub fn fresh(&self) -> bool {
		(self.created_at + chrono::Duration::seconds(self.expires_in - 15)) > chrono::Utc::now()
	}
}

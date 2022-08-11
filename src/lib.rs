#[cfg(feature = "hobo_plus")] pub mod hobo_plus;
pub mod error;
pub mod serde_utils;
pub mod common_prelude;
pub mod rkyv_shims;
pub mod duration;
pub mod logger;
pub mod math;

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

fn one_hour() -> i64 { 3600 }

#[derive(Debug, Serialize, Deserialize, Clone, smart_default::SmartDefault, PartialEq, Eq, Hash, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct OauthToken {
	#[serde(default = "one_hour")]
	pub expires_in: i64,
	pub access_token: String,
	pub refresh_token: Option<String>,
	#[with(rkyv_shims::ChronoDateTimeUtc)]
	#[default(chrono::Utc::now())]
	#[serde(default = "chrono::Utc::now")]
	pub created_at: chrono::DateTime<chrono::Utc>,
}

impl OauthToken {
	pub fn fresh(&self) -> bool {
		(self.created_at + chrono::Duration::seconds(self.expires_in - 15)) > chrono::Utc::now()
	}
}

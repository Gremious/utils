pub use core::future::Future;
use hobo::{create::components as cmp, enclose as e, events, prelude::*, state};
use wasm_bindgen_futures::spawn_local as spawn;
use once_cell::sync::Lazy;
use serde::{Serialize, Deserialize};

pub mod hobo_plus;
pub mod error;
pub mod debug;
// I don't like this name but I don't want it to conflict with actual serde and idk what to call it
pub mod serde_utils;

pub static REQWEST_CLIENT: Lazy<reqwest::Client> = Lazy::new(reqwest::Client::new);

pub fn window() -> web_sys::Window { web_sys::window().expect("no window") }
pub fn document() -> web_sys::Document { window().document().expect("no document") }

/* Prob don't need to keep? But also doen't a lot of things use a generic oauth token?
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
*/

pub fn spawn_complain<T>(x: impl Future<Output = anyhow::Result<T>> + 'static) {
	spawn(async move { if let Err(e) = x.await { log::error!("{:?}", e); } });
}

#[must_use]
pub fn default<T: Default>() -> T { T::default() }

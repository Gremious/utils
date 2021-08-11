use hobo::{create::components as cmp, enclose as e, events, prelude::*, state};
use wasm_bindgen_futures::spawn_local as spawn;
use once_cell::sync::Lazy;
use serde::{Serialize, Deserialize};

pub mod hobo_plus;
pub mod error;

pub static REQWEST_CLIENT: Lazy<reqwest::Client> = Lazy::new(reqwest::Client::new);

pub fn window() -> web_sys::Window { web_sys::window().expect("no window") }
pub fn document() -> web_sys::Document { window().document().expect("no document") }

// TODO: error type
pub trait SerdeJsonValueExt {
	fn from_pointer<T: serde::de::DeserializeOwned>(&self, pointer: &str) -> anyhow::Result<T>;
	fn from_pointer_mut<T: serde::de::DeserializeOwned>(&mut self, pointer: &str) -> anyhow::Result<T>;
}

impl SerdeJsonValueExt for serde_json::Value {
	fn from_pointer<T: serde::de::DeserializeOwned>(&self, pointer: &str) -> anyhow::Result<T> {
		use anyhow::Context;

		self
			.pointer(pointer)
			.context(format!("missing {}", pointer))
			.and_then(|x| Ok(serde_json::from_value(x.clone())?))
	}

	fn from_pointer_mut<T: serde::de::DeserializeOwned>(&mut self, pointer: &str) -> anyhow::Result<T> {
		use anyhow::Context;

		Ok(self
			.pointer_mut(pointer)
			.map(serde_json::Value::take)
			.map(serde_json::from_value)
			.context(format!("can't extract {}", pointer))??)
	}
}

pub mod chrono_duration_serde {
	use serde::{Deserialize, Serialize};
	pub fn deserialize<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<chrono::Duration, D::Error> {
		Ok(chrono::Duration::seconds(i64::deserialize(deserializer)?))
	}
	pub fn serialize<S: serde::Serializer>(value: &chrono::Duration, serializer: S) -> Result<S::Ok, S::Error> {
		i64::serialize(&value.num_seconds(), serializer)
	}
}

pub mod opt_chrono_duration_serde {
	use serde::{Deserialize, Serialize};
	pub fn deserialize<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<Option<chrono::Duration>, D::Error> {
		Ok(<Option<i64>>::deserialize(deserializer)?.map(chrono::Duration::seconds))
	}
	pub fn serialize<S: serde::Serializer>(value: &Option<chrono::Duration>, serializer: S) -> Result<S::Ok, S::Error> {
		<Option<i64>>::serialize(&value.map(|x| x.num_seconds()), serializer)
	}
}

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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use super::*;

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

#[async_trait::async_trait(?Send)]
trait ReqwestWasmResponseExt {
	async fn json<T: serde::de::DeserializeOwned>(self) -> anyhow::Result<T>;
}

#[async_trait::async_trait(?Send)]
impl ReqwestWasmResponseExt for reqwest::Response {
	async fn json<T: serde::de::DeserializeOwned>(self) -> anyhow::Result<T> {
		Ok(serde_json::from_str(&self.text().await?)?)
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

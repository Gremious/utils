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

pub mod chrono_duration {
	use serde::{Deserialize, Serialize};
	pub fn deserialize<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<chrono::Duration, D::Error> {
		Ok(chrono::Duration::milliseconds(i64::deserialize(deserializer)?))
	}
	pub fn serialize<S: serde::Serializer>(value: &chrono::Duration, serializer: S) -> Result<S::Ok, S::Error> {
		i64::serialize(&value.num_milliseconds(), serializer)
	}
}

pub mod opt_chrono_duration {
	use serde::{Deserialize, Serialize};
	pub fn deserialize<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<Option<chrono::Duration>, D::Error> {
		Ok(<Option<i64>>::deserialize(deserializer)?.map(chrono::Duration::milliseconds))
	}
	pub fn serialize<S: serde::Serializer>(value: &Option<chrono::Duration>, serializer: S) -> Result<S::Ok, S::Error> {
		<Option<i64>>::serialize(&value.map(|x| x.num_milliseconds()), serializer)
	}
}

pub fn string_or_number<'de, D: serde::Deserializer<'de>>(d: D) -> Result<u64, D::Error> {
	use serde::{Deserialize, de::Error};

	#[derive(Deserialize)]
	#[serde(untagged)]
	enum Value {
		String(String),
		U64(u64),
	}

	match Value::deserialize(d)? {
		Value::String(x) => x.parse::<u64>().map_err(|e| D::Error::custom(e.to_string())),
		Value::U64(x) => Ok(x),
	}
}

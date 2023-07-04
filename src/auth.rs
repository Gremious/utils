use super::*;

// Notes:
// Maybeee add a oauth_url() for an oatuh 2.0 flow somewhere?
// Maybe shared::oauth::SimpleCredentials belongs here?
// I don't want to overextend.

#[derive(Debug, Serialize, Deserialize, Clone, smart_default::SmartDefault, PartialEq, Eq, Hash, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct Token {
	#[serde(default = "one_hour")]
	pub expires_in: i64,
	pub access_token: String,
	pub refresh_token: Option<String>,
	#[with(rkyv_shims::ChronoDateTimeUtc)]
	#[default(chrono::Utc::now())]
	#[serde(default = "chrono::Utc::now")]
	pub created_at: chrono::DateTime<chrono::Utc>,
}

#[allow(unused_variables)]
pub trait SimpleToken {
	#[must_use] fn expires_in(&self) -> chrono::Duration;
	#[must_use] fn created_at(&self) -> chrono::DateTime<chrono::Utc>;
	#[must_use] fn access_token(&self) -> &str;
	#[must_use] fn refresh_token(&self) -> Option<&str> { None }
	// fn set_refresh_token(&mut self, token: Option<&str>) {}

	#[must_use] fn is_fresh(&self) -> bool {
		let created_at = self.created_at();
		let expires_in = self.expires_in();
		let quarter_lifetime = self.expires_in() / 4;

		(created_at + expires_in - quarter_lifetime) > chrono::Utc::now()
	}
}

impl SimpleToken for Token {
	fn expires_in(&self) -> chrono::Duration { chrono::Duration::seconds(self.expires_in) }
	fn created_at(&self) -> chrono::DateTime<chrono::Utc> { self.created_at }
	fn access_token(&self) -> &str { &self.access_token }
	fn refresh_token(&self) -> Option<&str> { self.refresh_token.as_ref().map(|x| x as &str) }
	// fn set_refresh_token(&mut self, token: Option<&str>) { if let Some(x) = token { self.refresh_token.replace(x.to_owned()); } }
}

/// Consider also implementing this for `&mut YourAuthStruct`,
/// because you'll likely be holding a reference to it from some state.
pub trait RefreshableToken: SimpleToken {
	// I think all tokens tend to have some sort of method to get them
	// so it wouldn't be a crazy idea to add a fn here like fn request_token(T) -> Result<Self>

	/// Refresh the token in place.
	async fn refresh(&mut self) -> anyhow::Result<()>;

	/// Refresh the token in place if not fresh.
	async fn ensure_fresh(&mut self) -> anyhow::Result<()> {
		if self.is_fresh() { return Ok(()); }
		if self.refresh_token().is_none() { anyhow::bail!("No refresh token."); }
		self.refresh().await
	}

	/// Get a fresh token.
	async fn fresh_token(&mut self) -> anyhow::Result<&str> {
		self.ensure_fresh().await?;
		Ok(self.access_token())
	}
}


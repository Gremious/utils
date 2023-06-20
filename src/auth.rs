// use super::*;
//
// #[derive(Debug, Serialize, Deserialize, Clone, smart_default::SmartDefault, PartialEq, Eq, Hash, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
// pub struct OauthToken {
//     #[serde(default = "one_hour")]
//     pub expires_in: i64,
//     pub access_token: String,
//     pub refresh_token: Option<String>,
//     #[with(rkyv_shims::ChronoDateTimeUtc)]
//     #[default(chrono::Utc::now())]
//     #[serde(default = "chrono::Utc::now")]
//     pub created_at: chrono::DateTime<chrono::Utc>,
// }

/// Consider also implementing this for `&mut YourAuthStruct`,
/// because you'll likely be holding a reference to it from some state.
pub trait AuthToken {
	// Maybeee add a oauth_url() for an oatuh 2.0 flow?

	fn expires_in(&self) -> chrono::Duration;
	fn created_at(&self) -> chrono::DateTime<chrono::Utc>;
	fn has_refresh_token(&self) -> bool;
	fn access_token(&self) -> &String;
	async fn refresh(&mut self) -> anyhow::Result<()>;

	#[must_use] fn is_fresh(&self) -> bool {
		let created_at = self.created_at();
		let expires_in = self.expires_in();
		let quarter_lifetime = self.expires_in() / 4;

		(created_at + expires_in - quarter_lifetime) > chrono::Utc::now()
	}

	async fn ensure_fresh(&mut self) -> anyhow::Result<()> {
		if self.is_fresh() { return Ok(()); }
		if !self.has_refresh_token() { anyhow::bail!("No refresh token."); }
		self.refresh().await
	}

	async fn fresh_token(&mut self) -> anyhow::Result<String> {
		self.ensure_fresh().await?;
		Ok(self.access_token().clone())
	}
}

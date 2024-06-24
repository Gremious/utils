#[derive(
	Clone, Copy, Debug, Hash,
	PartialOrd, Ord,
	PartialEq, Eq,
	shrinkwraprs::Shrinkwrap,
	Default,
	derive_more::Add, derive_more::Sub,
	derive_more::From, derive_more::Into,
	rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,
)]
#[repr(transparent)]
#[archive_attr(derive(PartialOrd, Ord, PartialEq, Eq))]
#[cfg_attr(feature = "chrono_hack", derive(serde::Serialize, serde::Deserialize))]
pub struct Duration(chrono::Duration);

impl std::fmt::Display for Duration {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.0.fmt(f) }
}

impl Duration {
	#[inline] pub fn zero()                          -> Self { Self(chrono::Duration::zero()) }
	#[inline] pub fn max_value()                     -> Self { Self(chrono::Duration::max_value()) }
	#[inline] pub fn weeks(weeks: i64)               -> Self { Self(chrono::Duration::weeks(weeks)) }
	#[inline] pub fn days(days: i64)                 -> Self { Self(chrono::Duration::days(days)) }
	#[inline] pub fn hours(hours: i64)               -> Self { Self(chrono::Duration::hours(hours)) }
	#[inline] pub fn minutes(minutes: i64)           -> Self { Self(chrono::Duration::minutes(minutes)) }
	#[inline] pub fn seconds(seconds: i64)           -> Self { Self(chrono::Duration::seconds(seconds)) }
	#[inline] pub fn milliseconds(milliseconds: i64) -> Self { Self(chrono::Duration::milliseconds(milliseconds)) }
	#[inline] pub fn microseconds(microseconds: i64) -> Self { Self(chrono::Duration::microseconds(microseconds)) }
	#[inline] pub fn nanoseconds(nanoseconds: i64)   -> Self { Self(chrono::Duration::nanoseconds(nanoseconds)) }

	pub fn as_seconds_f32(&self) -> f32 { self.as_seconds_f64() as _ }

	#[cfg(feature = "chrono_hack")]
	pub fn as_seconds_f64(&self) -> f64 {
		let (secs, nanos) = self.0.as_parts();
		secs as f64 + nanos as f64 / 1_000_000_000.
	}

	#[cfg(not(feature = "chrono_hack"))]
	pub fn as_seconds_f64(&self) -> f64 {
		let secs = self.0.num_seconds() as f64;
		let nanos = self.0.num_nanoseconds().unwrap_or(0) as f64;
		secs + nanos / 1_000_000_000.
	}


	pub fn seconds_f32(secs: f32) -> Self { Self::seconds_f64(secs as _) }
	pub fn seconds_f64(secs: f64) -> Self { Self(chrono::Duration::nanoseconds(f64::round(secs * 1_000_000_000.) as _)) }

	// https://en.wikipedia.org/wiki/Ramp_function
	#[must_use]
	pub fn ramp(self) -> Self { self.max(Self::zero()) }

	/// Returns a naive estimate of the number of whole years in the duration.
	#[inline]
	pub fn num_years_naive(&self) -> i64 {
		self.num_days() / 365
	}

	/// Pretty formatting in the stlye of "1 Day", "3 hours", etc. - whatever the largest denominator is.
	pub fn display_as_word(self) -> String {
		[
			(self.num_years_naive(),			   "year"),
			(self.num_weeks(),					   "week"),
			(self.num_days(),					   "day"),
			(self.num_hours(),					   "hour"),
			(self.num_minutes(),				   "minute"),
			(self.num_seconds(),				   "second"),
			(self.num_milliseconds(),			   "miliseconds"),
			(self.num_microseconds().unwrap_or(0), "microsecond"),
			(self.num_nanoseconds().unwrap_or(0),  "nanosecond"),
		].iter()
			.find(|(x, _)| *x > 0)
			.map_or("a moment".to_owned(), |(value, word)| format!("{value} {word}{}", if *value > 1 { "s" } else { "" }))
	}
}

impl TryFrom<std::time::Duration> for Duration {
	type Error = chrono::OutOfRangeError;
	fn try_from(dur: std::time::Duration) -> Result<Self, Self::Error> {
		Ok(Self(chrono::Duration::from_std(dur)?))
	}
}

impl TryFrom<Duration> for std::time::Duration {
	type Error = chrono::OutOfRangeError;
	fn try_from(dur: Duration) -> Result<Self, Self::Error> { dur.0.to_std() }
}

impl crate::hhmmss::Hhmmss for Duration { fn sms(&self) -> (i64, i64) { self.0.sms() } }

impl std::ops::Mul<f32> for Duration {
	type Output = Self;
	fn mul(self, rhs: f32) -> Self::Output { self * rhs as f64 }
}

impl std::ops::Mul<f64> for Duration {
	type Output = Self;
	fn mul(self, rhs: f64) -> Self::Output { Self::seconds_f64(self.as_seconds_f64() * rhs) }
}

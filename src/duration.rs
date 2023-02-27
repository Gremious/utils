#[derive(
	Clone, Copy, Debug, Hash,
	PartialOrd, Ord,
	PartialEq, Eq,
	shrinkwraprs::Shrinkwrap,
	smart_default::SmartDefault,
	serde::Serialize, serde::Deserialize,
	derive_more::Add, derive_more::Sub,
	derive_more::From, derive_more::Into,

)]
#[repr(transparent)]
pub struct Duration(
	#[serde(with = "crate::serde_utils::chrono_duration")]
	#[default(chrono::Duration::zero())]
	chrono::Duration,
);

impl std::fmt::Display for Duration {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.0.fmt(f) }
}

impl Duration {
	pub fn zero() -> Self { Self(chrono::Duration::zero()) }
	pub fn max_value() -> Self { Self(chrono::Duration::max_value()) }
	#[inline] pub fn weeks(weeks: i64) -> Self { Self(chrono::Duration::weeks(weeks)) }
	#[inline] pub fn days(days: i64) -> Self { Self(chrono::Duration::days(days)) }
	#[inline] pub fn hours(hours: i64) -> Self { Self(chrono::Duration::hours(hours)) }
	#[inline] pub fn minutes(minutes: i64) -> Self { Self(chrono::Duration::minutes(minutes)) }
	#[inline] pub fn seconds(seconds: i64) -> Self { Self(chrono::Duration::seconds(seconds)) }
	#[inline] pub fn milliseconds(milliseconds: i64) -> Self { Self(chrono::Duration::milliseconds(milliseconds)) }
	#[inline] pub fn microseconds(microseconds: i64) -> Self { Self(chrono::Duration::microseconds(microseconds)) }
	#[inline] pub fn nanoseconds(nanoseconds: i64) -> Self { Self(chrono::Duration::nanoseconds(nanoseconds)) }

	pub fn as_seconds_f32(&self) -> f32 {
		let secs = self.num_seconds();
		let dur = Self::seconds(secs);
		let nanos = *self - dur;
		let nanos = nanos.num_nanoseconds().unwrap();
		secs as f32 + nanos as f32 / 1_000_000_000.
	}

	pub fn as_seconds_f64(&self) -> f64 {
		let secs = self.num_seconds();
		let dur = Self::seconds(secs);
		let nanos = *self - dur;
		let nanos = nanos.num_nanoseconds().unwrap();
		secs as f64 + nanos as f64 / 1_000_000_000.
	}
	pub fn seconds_f32(secs: f32) -> Self { Self(chrono::Duration::milliseconds(f32::round(secs * 1000.) as _)) }
	pub fn seconds_f64(secs: f64) -> Self { Self(chrono::Duration::milliseconds(f64::round(secs * 1000.) as _)) }

	// https://en.wikipedia.org/wiki/Ramp_function
	#[must_use]
	pub fn ramp(self) -> Self { self.max(Self::zero()) }

    /// Returns a naive estimate of the number of whole years in the duration.
    #[inline]
    pub fn num_years_naive(&self) -> i64 {
        self.num_days() / 365
    }

	/// Pretty formatting in the stlye of "1 Day", "3 hours", etc. - whatever the largest denominator is.
	// made me feel like i'm writing a joke yandere dev would make, but I don't thiiiink there's a better way??
	// I could google for a crate but it's like 1 fn, I might as well be the one
	// also idk how to name this well
	pub fn display_as_word(self) -> String {
		return if self.num_years_naive() > 1 {
			let years = self.num_years_naive();
			format!("{years} year{}", if years >= 2 {"s"} else {""})
		} else if self.num_weeks() > 0 {
			let weeks = self.num_weeks();
			format!("{weeks} week{}", if weeks >= 2 {"s"} else {""})
		} else if self.num_days() > 0 {
			let days = self.num_days();
			format!("{days} day{}", if days >= 2 {"s"} else {""})
		} else if self.num_hours() > 0 {
			let hours = self.num_hours();
			format!("{hours} hour{}", if hours >= 2 {"s"} else {""})
		} else if self.num_minutes() > 0 {
			let minutes = self.num_minutes();
			format!("{minutes} minute{}", if minutes >= 2 {"s"} else {""})
		} else if self.num_seconds() > 0 {
			let seconds = self.num_seconds();
			format!("{seconds} second{}", if seconds >= 2 {"s"} else {""})
		} else if self.num_milliseconds() > 0 {
			let milliseconds = self.num_milliseconds();
			format!("{milliseconds} millisecond{}", if milliseconds >= 2 {"s"} else {""})
		} else if self.num_microseconds() > Some(0) {
			let microseconds = self.num_microseconds().unwrap();
			format!("{microseconds} microsecond{}", if microseconds >= 2 {"s"} else {""})
		} else if self.num_nanoseconds() > Some(0) {
			let nanoseconds = self.num_nanoseconds().unwrap();
			format!("{nanoseconds} nanosecond{}", if nanoseconds >= 2 {"s"} else {""})
		} else {
			String::from("a moment")
		}
	}
}

impl crate::hhmmss::Hhmmss for Duration { fn sms(&self) -> (i64, i64) { self.0.sms() } }

impl std::ops::Mul<f32> for Duration {
	type Output = Self;

	fn mul(self, rhs: f32) -> Self::Output {
		Self::seconds_f32(self.as_seconds_f32() * rhs)
	}
}

impl rkyv::Archive for Duration {
	type Archived = Duration;
	type Resolver = ((), ());

	#[inline]
	unsafe fn resolve(&self, _: usize, _: ((), ()), out: *mut Self::Archived) {
		out.write(*self);
	}
}

impl<D: rkyv::Fallible + ?Sized> rkyv::Deserialize<Duration, D> for rkyv::Archived<Duration> {
	#[inline]
	fn deserialize(&self, _: &mut D) -> Result<Duration, D::Error> {
		Ok(*self)
	}
}

// TODO: HACK:
// this is kinda bad and probably UB? because chrono's Duration is not repr(transparent)
impl<S: rkyv::Fallible + ?Sized> rkyv::Serialize<S> for Duration {
	#[inline]
	fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
		rkyv::Serialize::serialize(unsafe { std::mem::transmute::<_, &(i64, i32)>(self) }, serializer)
	}
}

#[test]
fn rkyv_test() {
	let duration = Duration(chrono::Duration::milliseconds(15726));
	let bytes = rkyv::to_bytes::<_, 256>(&duration).unwrap();
	println!("bytes: {bytes:?}");
	let archived = unsafe { rkyv::archived_root::<Duration>(&bytes) };
	println!("archived: {archived:?}");
	let duration_again: Duration = rkyv::Deserialize::deserialize(archived, &mut rkyv::Infallible).unwrap();
	assert_eq!(duration, duration_again);
}

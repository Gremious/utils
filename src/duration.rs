#[derive(
	Clone, Copy, Debug, Hash,
	PartialOrd, Ord,
	PartialEq, Eq,
	shrinkwraprs::Shrinkwrap,
	smart_default::SmartDefault,
	serde::Serialize, serde::Deserialize,
	derive_more::Display,
	derive_more::Add, derive_more::Sub,
	derive_more::From, derive_more::Into,

)]
#[repr(transparent)]
pub struct Duration(
	#[serde(with = "crate::serde_utils::chrono_duration")]
	#[default(chrono::Duration::zero())]
	chrono::Duration,
);

impl Duration {
	pub fn zero() -> Self { Self::default() }
	#[inline] pub fn weeks(weeks: i64) -> Self { chrono::Duration::weeks(weeks).into() }
	#[inline] pub fn days(days: i64) -> Self { chrono::Duration::days(days).into() }
	#[inline] pub fn hours(hours: i64) -> Self { chrono::Duration::hours(hours).into() }
	#[inline] pub fn minutes(minutes: i64) -> Self { chrono::Duration::minutes(minutes).into() }
	#[inline] pub fn seconds(seconds: i64) -> Self { chrono::Duration::seconds(seconds).into() }
	#[inline] pub fn milliseconds(milliseconds: i64) -> Self { chrono::Duration::milliseconds(milliseconds).into() }
	#[inline] pub fn microseconds(microseconds: i64) -> Self { chrono::Duration::microseconds(microseconds).into() }
	#[inline] pub fn nanoseconds(nanoseconds: i64) -> Self { chrono::Duration::nanoseconds(nanoseconds).into() }

	pub fn as_seconds_f32(&self) -> f32 {
		let secs = self.num_seconds();
		let dur = Self::seconds(secs);
		let nanos = *self - dur;
		let nanos = nanos.num_nanoseconds().unwrap();
        secs as f32 + nanos as f32 * 1_000_000_000.
	}

	pub fn as_seconds_f64(&self) -> f64 {
		let secs = self.num_seconds();
		let dur = Self::seconds(secs);
		let nanos = *self - dur;
		let nanos = nanos.num_nanoseconds().unwrap();
        secs as f64 + nanos as f64 * 1_000_000_000.
    }
	pub fn seconds_f32(secs: f32) -> Self { Self(chrono::Duration::milliseconds(f32::round(secs * 1000.) as _)) }
	pub fn seconds_f64(secs: f64) -> Self { Self(chrono::Duration::milliseconds(f64::round(secs * 1000.) as _)) }

	pub fn mmss(&self) -> String { format!("{:02}:{:02}", self.num_minutes(), self.num_seconds() % 60) }
}

impl hhmmss::Hhmmss for Duration {
	fn sms(&self) -> (i64, i64) {
		<chrono::Duration as hhmmss::Hhmmss>::sms(self)
	}
}

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

impl<S: rkyv::Fallible + ?Sized> rkyv::Serialize<S> for Duration {
	#[inline]
	fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
		rkyv::Serialize::serialize(unsafe { std::mem::transmute::<_, &(i64, i32)>(self) }, serializer)
	}
}

#[test]
fn rkyv_test() {
	let duration = Duration(chrono::Duration::seconds(15));
	let bytes = rkyv::to_bytes::<_, 256>(&duration).unwrap();
	println!("bytes: {bytes:?}");
	let archived = unsafe { rkyv::archived_root::<Duration>(&bytes) };
	println!("archived: {archived:?}");
	let duration_again: Duration = rkyv::Deserialize::deserialize(archived, &mut rkyv::Infallible).unwrap();
	assert_eq!(duration, duration_again);
}

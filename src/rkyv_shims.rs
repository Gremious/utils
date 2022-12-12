#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
#[archive_attr(derive(Debug))]
pub struct Duration {
	secs: i64,
	nanos: i32,
}

impl rkyv::with::ArchiveWith<chrono::Duration> for Duration {
	type Archived = rkyv::Archived<Duration>;
	type Resolver = rkyv::Resolver<Duration>;

	unsafe fn resolve_with(field: &chrono::Duration, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
		rkyv::Archive::resolve(std::mem::transmute::<_, &Duration>(field), pos, resolver, out);
	}
}

impl<S: rkyv::Fallible + ?Sized> rkyv::with::SerializeWith<chrono::Duration, S> for Duration {
	fn serialize_with(field: &chrono::Duration, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
		rkyv::Serialize::serialize(unsafe { std::mem::transmute::<_, &Duration>(field) }, serializer)
	}
}

impl<D: rkyv::Fallible + ?Sized> rkyv::with::DeserializeWith<rkyv::Archived<Duration>, chrono::Duration, D> for Duration where
	rkyv::Archived<Duration>: rkyv::Deserialize<Duration, D>,
{
	fn deserialize_with(field: &rkyv::Archived<Duration>, deserializer: &mut D) -> Result<chrono::Duration, D::Error> {
		Ok(unsafe { std::mem::transmute(rkyv::Deserialize::deserialize(field, deserializer)?) })
	}
}

pub struct OptionDuration;

impl rkyv::with::ArchiveWith<Option<chrono::Duration>> for OptionDuration {
	type Archived = rkyv::Archived<Option<Duration>>;
	type Resolver = rkyv::Resolver<Option<Duration>>;

	unsafe fn resolve_with(field: &Option<chrono::Duration>, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
		rkyv::Archive::resolve(std::mem::transmute::<_, &Option<Duration>>(field), pos, resolver, out);
	}
}

impl<S: rkyv::Fallible + ?Sized> rkyv::with::SerializeWith<Option<chrono::Duration>, S> for OptionDuration {
	fn serialize_with(field: &Option<chrono::Duration>, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
		rkyv::Serialize::serialize(unsafe { std::mem::transmute::<_, &Option<Duration>>(field) }, serializer)
	}
}

impl<D: rkyv::Fallible + ?Sized> rkyv::with::DeserializeWith<rkyv::Archived<Option<Duration>>, Option<chrono::Duration>, D> for OptionDuration where
	rkyv::Archived<Option<Duration>>: rkyv::Deserialize<Option<Duration>, D>,
{
	fn deserialize_with(field: &rkyv::Archived<Option<Duration>>, deserializer: &mut D) -> Result<Option<chrono::Duration>, D::Error> {
		Ok(unsafe { std::mem::transmute(rkyv::Deserialize::deserialize(field, deserializer)?) })
	}
}

pub struct ChronoDateTimeUtc;

impl rkyv::with::ArchiveWith<chrono::DateTime<chrono::Utc>> for ChronoDateTimeUtc {
	type Archived = rkyv::Archived<i64>;
	type Resolver = rkyv::Resolver<i64>;

	unsafe fn resolve_with(field: &chrono::DateTime<chrono::Utc>, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
		rkyv::Archive::resolve(&field.timestamp(), pos, resolver, out);
	}
}

impl<S: rkyv::Fallible + ?Sized> rkyv::with::SerializeWith<chrono::DateTime<chrono::Utc>, S> for ChronoDateTimeUtc {
	fn serialize_with(field: &chrono::DateTime<chrono::Utc>, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
		rkyv::Serialize::serialize(&field.timestamp(), serializer)
	}
}

impl<D: rkyv::Fallible + ?Sized> rkyv::with::DeserializeWith<rkyv::Archived<i64>, chrono::DateTime<chrono::Utc>, D> for ChronoDateTimeUtc where
	rkyv::Archived<i64>: rkyv::Deserialize<i64, D>,
{
	fn deserialize_with(field: &rkyv::Archived<i64>, deserializer: &mut D) -> Result<chrono::DateTime<chrono::Utc>, D::Error> {
		Ok(chrono::DateTime::<chrono::Utc>::from_utc(chrono::NaiveDateTime::from_timestamp_opt(rkyv::Deserialize::deserialize(field, deserializer)?, 0).unwrap(), chrono::Utc))
	}
}

pub struct ChronoNaiveDate;

impl rkyv::with::ArchiveWith<chrono::NaiveDate> for ChronoNaiveDate {
	type Archived = rkyv::Archived<i64>;
	type Resolver = rkyv::Resolver<i64>;

	unsafe fn resolve_with(field: &chrono::NaiveDate, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
		rkyv::Archive::resolve(&field.and_hms_opt(0, 0, 0).unwrap().timestamp(), pos, resolver, out);
	}
}

impl<S: rkyv::Fallible + ?Sized> rkyv::with::SerializeWith<chrono::NaiveDate, S> for ChronoNaiveDate {
	fn serialize_with(field: &chrono::NaiveDate, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
		rkyv::Serialize::serialize(&field.and_hms_opt(0, 0, 0).unwrap().timestamp(), serializer)
	}
}

impl<D: rkyv::Fallible + ?Sized> rkyv::with::DeserializeWith<rkyv::Archived<i64>, chrono::NaiveDate, D> for ChronoNaiveDate where
	rkyv::Archived<i64>: rkyv::Deserialize<i64, D>,
{
	fn deserialize_with(field: &rkyv::Archived<i64>, deserializer: &mut D) -> Result<chrono::NaiveDate, D::Error> {
		Ok(chrono::NaiveDateTime::from_timestamp_opt(rkyv::Deserialize::deserialize(field, deserializer)?, 0).unwrap().date())
	}
}

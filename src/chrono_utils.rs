use chrono::prelude::*;

#[extend::ext(pub, name = ChronoNaiveDateExt)]
impl chrono::NaiveDate {
	fn monthly_after(&self, other: chrono::NaiveDate) -> chrono::NaiveDate {
		let other_month_has_my_day = other.with_day0(self.day0()).is_some();
		if other_month_has_my_day {
			if self.day0() <= other.day0() {
				other.with_day0(self.day0()).unwrap() + chrono::Months::new(1)
			} else {
				other.with_day0(self.day0()).unwrap()
			}
		} else {
			(other.with_day0(0).unwrap() + chrono::Months::new(1)).pred_opt().unwrap()
		}
	}
}

#[test]
fn t1() {
	let a = chrono::NaiveDate::from_ymd(2023, 5, 5);
	let b = chrono::NaiveDate::from_ymd(2024, 1, 1);
	let c = chrono::NaiveDate::from_ymd(2024, 1, 5);
	assert_eq!(a.monthly_after(b), c);
}

#[test]
fn t2() {
	let a = chrono::NaiveDate::from_ymd(2023, 5, 1);
	let b = chrono::NaiveDate::from_ymd(2024, 1, 5);
	let c = chrono::NaiveDate::from_ymd(2024, 2, 1);
	assert_eq!(a.monthly_after(b), c);
}

#[test]
fn t3() {
	let a = chrono::NaiveDate::from_ymd(2023, 8, 31);
	let b = chrono::NaiveDate::from_ymd(2024, 2, 1);
	let c = chrono::NaiveDate::from_ymd(2024, 2, 29);
	assert_eq!(a.monthly_after(b), c);
}

#[test]
fn t4() {
	let a = chrono::NaiveDate::from_ymd(2022, 8, 31);
	let b = chrono::NaiveDate::from_ymd(2023, 2, 1);
	let c = chrono::NaiveDate::from_ymd(2023, 2, 28);
	assert_eq!(a.monthly_after(b), c);
}

#[test]
fn t5() {
	let a = chrono::NaiveDate::from_ymd(2023, 8, 31);
	let b = chrono::NaiveDate::from_ymd(2024, 8, 31);
	let c = chrono::NaiveDate::from_ymd(2024, 9, 30);
	assert_eq!(a.monthly_after(b), c);
}

#[test]
fn t6() {
	let a = chrono::NaiveDate::from_ymd(2023, 9, 1);
	let b = chrono::NaiveDate::from_ymd(2024, 9, 1);
	let c = chrono::NaiveDate::from_ymd(2024, 10, 1);
	assert_eq!(a.monthly_after(b), c);
}

#[test]
fn t7() {
	let a = chrono::NaiveDate::from_ymd(2023, 9, 1);
	let b = chrono::NaiveDate::from_ymd(2024, 8, 31);
	let c = chrono::NaiveDate::from_ymd(2024, 9, 1);
	assert_eq!(a.monthly_after(b), c);
}

#[test]
fn t8() {
	let a = chrono::NaiveDate::from_ymd(2023, 9, 15);
	let b = chrono::NaiveDate::from_ymd(2024, 12, 15);
	let c = chrono::NaiveDate::from_ymd(2025, 1, 15);
	assert_eq!(a.monthly_after(b), c);
}

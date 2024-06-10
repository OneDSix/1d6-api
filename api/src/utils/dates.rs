use chrono::{Datelike, Local};

#[derive(PartialEq, Debug)]
pub enum SpecialDates {
	OneDSixBirthday,
	AprilFools,
	Halloween,
	Christmas,
	NewYears,
	NotSpecial
}

impl SpecialDates {
	pub fn is_special() -> SpecialDates {
		let now = Local::now();
		match (now.month(), now.day()) {
			(1, 24) => SpecialDates::OneDSixBirthday,
			(4, 1) => SpecialDates::AprilFools,
			(10, 31) => SpecialDates::Halloween,
			(12, 24) => SpecialDates::Christmas,
			(12, 25) => SpecialDates::Christmas,
			(12, 31) => SpecialDates::NewYears,
			(1, 1) => SpecialDates::NewYears,
			_ => SpecialDates::NotSpecial,
		}
	}
}

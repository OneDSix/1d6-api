#![allow(unused)]

use chrono::{DateTime, Duration, Utc};

pub struct Cache<T> {
    data: Option<T>,
    last_updated: DateTime<Utc>,
    max_age: i64,
    // I'm aware this is a wasteful allocation but Duration::hours() errors out otherwise.
    // plus the u32, i32, and u8 conversions are probably too expensive for this struct
}

impl<T> Cache<T> {

    const MAX_AGE_HOURS: i64 = 3;

    fn new() -> Self {
        Cache {
            data: None,
            last_updated: Utc::now() - Duration::hours(Self::MAX_AGE_HOURS + 1), // Ensure it's expired initially
            max_age: Self::MAX_AGE_HOURS,
        }
    }

	fn new_full(input_data: T, hours: i64) -> Self {
		Cache {
			data: Some(input_data),
			last_updated: Utc::now() - Duration::hours(hours + 1),
			max_age: hours,
		}
	}

    fn is_expired(&self) -> bool {
        Utc::now() - self.last_updated >= Duration::hours(self.max_age)
    }

	/// You must check if the cache is expired using is_expired() first, as running this will overwrite the data!
    fn update(&mut self, data: T) {
		self.data = Some(data);
		self.last_updated = Utc::now();
    }

	fn set_expiration(&mut self, hours: i64) {
		self.max_age = hours;
	}
}

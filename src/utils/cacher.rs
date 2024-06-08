#![allow(unused)]

use chrono::{DateTime, Duration, Utc};

pub struct Cache<T> {
    pub cache_data: Option<T>,
    pub last_updated: DateTime<Utc>,
    pub max_age: i64,
}

impl<T> Cache<T> {

    const DEFAULT_MAX_AGE_HOURS: i64 = 3;

	/// Creates an empty cache
    pub fn new() -> Self {
        Self {
            cache_data: None,
            last_updated: Utc::now() - Duration::hours(Self::DEFAULT_MAX_AGE_HOURS + 1), // Ensure it's expired initially
            max_age: Self::DEFAULT_MAX_AGE_HOURS,
        }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() - self.last_updated >= Duration::hours(self.max_age)
    }

	/// You must check if the cache is expired using is_expired() first, as running this will overwrite the data!
    pub fn update(&mut self, data: T) {
		self.cache_data = Some(data);
		self.last_updated = Utc::now();
    }

	/// As apposed to update(), this one checks if the recache is valid first.
	pub fn safe_update(&mut self, data: T) {
		if self.is_expired() {
			self.cache_data = Some(data);
			self.last_updated = Utc::now();
			println!("Recached!")
		}
    }

	pub fn set_expiration(&mut self, hours: i64) {
		self.max_age = hours;
	}

	pub fn get_expiration(&self) -> i64 {
        let total_age = self.last_updated.clone() + Duration::hours(self.max_age);
        let now = Utc::now();
        let duration_until_refresh = now - total_age;

        // Ensure the duration is positive
        //if duration_until_refresh <= Duration::zero() {
        //    return 0;
        //}

        // Convert to seconds for readability
        duration_until_refresh.num_seconds()
    }
}

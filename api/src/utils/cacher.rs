#![allow(unused)]

use std::collections::HashMap;

use chrono::{DateTime, Duration, Utc};

const DEFAULT_MAX_AGE_HOURS: i64 = 3;

/// Common functions between both caching structs.
pub trait Caching<T> {
	fn new() -> Self;
	fn is_expired(&self) -> bool;
	fn get_expiration(&self) -> i64;
	fn unsafe_update(&mut self, data: T);
	fn update(&mut self, data: T);
}

/// A simple single value cache. Useful for storing small, focused, frequent API calls.<br>
/// Currently used for caching statistics, as those database calls can be very expensive.<br>
/// TODO: Migrate this to use `i64` or `f64` instead of `DateTime<UTC>`
#[derive(Clone, Debug)]
pub struct Cache<T> {
    pub cache_data: Option<T>,
    pub last_updated: DateTime<Utc>,
    pub max_age: i64,
}

impl<T> Caching<T> for Cache<T> {

	/// Creates an empty cache
    fn new() -> Self {
        Self {
            cache_data: None,
            last_updated: Utc::now() - Duration::hours(DEFAULT_MAX_AGE_HOURS + 1), // Ensure it's expired initially
            max_age: DEFAULT_MAX_AGE_HOURS,
        }
    }

    fn is_expired(&self) -> bool {
        Utc::now() - self.last_updated >= Duration::hours(self.max_age)
    }

	fn get_expiration(&self) -> i64 {
        let total_age = self.last_updated.clone() + Duration::hours(self.max_age);
        let now = Utc::now();
        let duration_until_refresh = now - total_age;
        duration_until_refresh.num_seconds()
    }

	/// Overwrites any data present, no matter what.
    fn unsafe_update(&mut self, data: T) {
		self.cache_data = Some(data);
		self.last_updated = Utc::now();
    }

	/// Checks if the value needs updating, and if you replaces it.
	fn update(&mut self, data: T) {
		if self.is_expired() {
			self.cache_data = Some(data);
			self.last_updated = Utc::now();
		}
    }
}

/// Gives the ability to cache a lot of data, usually in the case of expensive search or database calls.<br>
/// Used for caching frequent searches and database accesses for mods and servers.<br>
/// TODO: Migrate this to use `i64` or `f64` instead of `DateTime<UTC>`
#[derive(Clone, Debug)]
struct MultiCache<T> {
	pub cache_data: Vec<T>,
    pub last_updated: DateTime<Utc>,
    pub max_age: i64,
}

impl<T> Caching<T> for MultiCache<T> {

	/// Creates an empty cache
	fn new() -> Self {
		Self {
			cache_data: Vec::new(),
			last_updated: Utc::now() - Duration::hours(DEFAULT_MAX_AGE_HOURS + 1), // Ensure it's expired initially
			max_age: DEFAULT_MAX_AGE_HOURS,
		}
	}

	fn is_expired(&self) -> bool {
		Utc::now() - self.last_updated >= Duration::hours(self.max_age)
	}

	fn get_expiration(&self) -> i64 {
		let total_age = self.last_updated.clone() + Duration::hours(self.max_age);
		let now = Utc::now();
		let duration_until_refresh = now - total_age;
		duration_until_refresh.num_seconds()
	}

	/// You must check if the cache is expired using is_expired() first, as running this will overwrite the data!
	fn unsafe_update(&mut self, data: T) {
		self.cache_data.push(data);
		self.last_updated = Utc::now();
	}

	/// Checks if the value needs updating, and if you replaces it.
	fn update(&mut self, data: T) {
		if self.is_expired() {
			self.cache_data.push(data);
			self.last_updated = Utc::now();
		}
	}
}

impl<T> MultiCache<T> {
	/// Expires (removes) all data in the cache.
	pub fn expire_all(&mut self) {
		self.cache_data = Vec::new()
	}

	/// Expire only data in need of being expired.
	pub fn expire_expired(&mut self) {
		for data in &self.cache_data {
			// TODO
		}
	}
}

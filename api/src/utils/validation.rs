use actix_web::web::Data;
use actix_identity::Identity;
use censor::Censor;

use crate::{routes::errors::ApiErrors, AppState};

struct SqlInjection;

impl SqlInjection {
	const INJECTION_ARRAY: [&'static str; 28] = [
		"SELECT", "INSERT", "UPDATE", "DELETE",
		"DROP", "ALTER", "UNION", "JOIN",
		"FROM", "WHERE", "HAVING", "OR",
		"AND", "LIKE", "GROUP", "ORDER",
		"BY", "EXEC", "EXECUTE", "DECLARE",
		"TRUNCATE", "RENAME", "CREATE", "TABLE",
		"INDEX", "VIEW", "PROCEDURE", "FUNCTION",
	];

	pub fn sql_injection(username: String) -> Result<(), ()> {
		for &item in Self::INJECTION_ARRAY.iter() {
			if username.to_uppercase().contains(item) {
				return Err(())
			}
		};
		Ok(())
	}
}

/// An enum for checking if the user is signed in, using `actix_identity::Identity`.<br>
/// Returns the `Identity` if there is a valid `Identity` present, otherwise returns `AuthChecker::NoLI`.<br>
/// This will be moved to a middleware eventually, maybe with API V2.
pub enum AuthChecker {
    NoLI, // NOt Logged In
    Success(Identity),
	Unauthorized
}

impl AuthChecker {
    pub fn check_exists(identity: Option<Identity>) -> Result<Self, Self> {
        match identity.map(|id| id) {
            None => return Err(Self::NoLI),
            Some(id) => return Ok(Self::Success(id)),
        }
    }

	pub fn check_against(identity: Option<Identity>, target: &String) -> Result<Self, Self> {
		match identity {
            None => Err(Self::NoLI),
            Some(id) => match id.id() {
				Ok(current) => {
					if current == *target {
						Ok(Self::Success(id))
					} else {
						Err(Self::Unauthorized)}
					},
				_ => Err(Self::NoLI),
			},
        }
	}

	pub fn get_identity(self) -> Option<Identity> {
        match self {
            Self::Success(identity) => Some(identity),
            Self::NoLI => None,
			Self::Unauthorized => None,
        }
    }
}

pub enum UsernameResult {
	DatabaseError(String),
	Taken,
	FowlLanguage,
	SqlInjection,
	Passed,
	UnhandledResult
}

impl From<ApiErrors<'_>> for UsernameResult {
    fn from(error: ApiErrors) -> Self {
        match error {
            ApiErrors::DatabaseError(error_msg) => UsernameResult::DatabaseError(error_msg),
            _ => UsernameResult::UnhandledResult,
        }
    }
}

impl UsernameResult {
	pub async fn username_check(username: String, state: &Data<AppState>) -> Result<UsernameResult, UsernameResult> {

		let query: Result<(bool,), Self> = sqlx::query_as(
			"SELECT EXISTS (SELECT 1 FROM users WHERE username = username)"
		)
		.bind(&username)
		.fetch_one(&state.pool)
		.await
		.map_err(|e| Self::from(ApiErrors::DatabaseError(e.to_string())));

		if let Ok((exists,)) = query {
			if exists == true {
				return Err(Self::Taken);
			}
		}

		if Censor::Sex.check(&username) || Censor::Standard.check(&username) {
			return Err(Self::FowlLanguage)
		}

		if SqlInjection::sql_injection(username).is_err() {
			return Err(Self::SqlInjection)
		}

		Ok(Self::Passed)
	}
}

#[derive(PartialEq)]
pub enum PasswordResult {
	SHA256,
	SHA512,
	Argon2,
	UnknownHash
}

impl PasswordResult {
	pub async fn password_check(password: String) -> Result<PasswordResult, PasswordResult> {
		match &password.len() {
			64 if Self::is_valid_hex(&password) && !&password.contains(' ') => Ok(Self::SHA256),
			128 if Self::is_valid_hex(&password) && !&password.contains(' ') => Ok(Self::SHA512),
			_ => Ok(
				if password.starts_with("$argon2") {
					Self::Argon2
				} else {
					Self::UnknownHash
				}
			)
		}
	}

	fn is_valid_hex(s: &String) -> bool {
		s.chars().all(|c| c.is_digit(16))
	}
}

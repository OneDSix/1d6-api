use actix_identity::Identity;

/// A single function enum for checking if the user is signed in, using `actix_identity::Identity`.<br>
/// Returns the `Identity` if there is a valid `Identity` present, otherwise returns `AuthChecker::NoLI`.<br>
/// This will be moved to a middleware eventually, maybe with API V2.
pub enum AuthChecker {
    NoLI, // NOt Logged In
    Success(Identity),
}

impl AuthChecker {
    pub fn check_auth(identity: Option<Identity>) -> Result<Self, Self> {
        match identity.map(|id| id) {
            None => return Err(Self::NoLI),
            Some(id) => return Ok(Self::Success(id)),
        }
    }
}

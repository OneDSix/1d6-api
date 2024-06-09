use std::pin::Pin;

use actix_identity::Identity;
use actix_web::HttpRequest;
use futures::Future;

use crate::ApiErrors;

/// A struct for checking if the user is signed in, using `actix_identity::Identity`.<br>
/// Returns the `Identity` if there is a valid `Identity` present, otherwise returns `AuthChecker::NoLI`.<br>
/// This will be moved to a more sophisticated format eventually, maybe with API V2.
///
/// ## Example
/// ```
/// async fn request_handler(identity: Option<Identity>, req: HttpRequest) -> Result<HttpRequest, ApiErrors> {
///		AuthChecker::auth_switcher(
///			|id: Identity, (req, json_data): (HttpRequest, Option<serde_json::Value>)| {
///				// Code for a successful authentication here
///				HttpResponse::Ok().json(json!({"success":true}))
///			},
///			|(req, json_data): (HttpRequest, Option<serde_json::Value>)| {
///				// Code for NoLIs here
/// 			ApiErrors::Unauthorized.error_response()
///			},
///			identity,
///			(req, None),
/// 	).await
/// }
/// ```
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

    pub async fn auth_switcher<Auth, NoLI, Args>(
        authenticated: Auth,
        noli: NoLI,
        ident: Option<Identity>,
        args: Args,
    ) -> Result<HttpRequest, ApiErrors>
    where
        Auth: Fn(Identity, Args) -> Pin<Box<dyn Future<Output = Result<HttpRequest, ApiErrors>> + Send + Sync + 'static>>,
        NoLI: Fn(Args) -> Pin<Box<dyn Future<Output = Result<HttpRequest, ApiErrors>> + Send + Sync + 'static>>,
        Args: Clone + 'static,
    {
        if let Some(id) = ident {
            return authenticated(id, args).await;
        } else {
            return noli(args).await;
        }
    }
}

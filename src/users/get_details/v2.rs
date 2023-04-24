//! [GET /_synapse/admin/v2/users/:user_id](https://github.com/matrix-org/synapse/blob/master/docs/admin_api/user_admin_api.rst#query-user-account)

pub use crate::users::UserDetails;
use ruma::{
    api::{metadata, request, response, Metadata},
    OwnedUserId,
};

const METADATA: Metadata = metadata! {
    method: GET,
    rate_limited: false,
    authentication: AccessToken,
    history: {
        unstable => "/_synapse/admin/v2/users/:user_id",
    }
};

#[request]
pub struct Request {
    /// user ID
    #[ruma_api(path)]
    pub user_id: OwnedUserId,
}

#[response]
pub struct Response {
    /// Details about the user.
    #[ruma_api(body)]
    pub details: UserDetails,
}

impl Request {
    /// Creates an `Request` with the given user ID.
    pub fn new(user_id: OwnedUserId) -> Self {
        Self { user_id }
    }
}

impl Response {
    /// Creates a new `Response` with all parameters defaulted.
    pub fn new(details: UserDetails) -> Self {
        Self { details }
    }
}

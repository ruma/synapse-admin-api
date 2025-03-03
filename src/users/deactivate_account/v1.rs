//! [POST /_synapse/admin/v1/deactivate/:user_id](https://github.com/matrix-org/synapse/blob/develop/docs/admin_api/user_admin_api.rst#deactivate-account)

use ruma::{
    api::{metadata, request, response, Metadata},
    UserId,
};

const METADATA: Metadata = metadata! {
    method: POST,
    rate_limited: false,
    authentication: AccessToken,
    history: {
        unstable => "/_synapse/admin/v1/deactivate/:user_id",
    }
};

#[request]
pub struct Request {
    /// User ID
    #[ruma_api(path)]
    pub user_id: OwnedUserId,

    /// Flag whether to erase the account.
    #[serde(default = "ruma::serde::default_false", skip_serializing_if = "ruma::serde::is_false")]
    pub erase: bool,
}

#[derive(Default)]
#[response]
pub struct Response {}

impl Request {
    /// Creates a `Request` with the given user ID.
    pub fn new(user_id: OwnedUserId) -> Self {
        Self { user_id }
    }
}

impl Response {
    /// Creates an empty `Response`.
    pub fn new() -> Self {
        Self
    }
}

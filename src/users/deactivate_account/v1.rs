//! [POST /_synapse/admin/v1/deactivate/:user_id](https://github.com/element-hq/synapse/blob/develop/docs/admin_api/user_admin_api.md#deactivate-account)

use ruma::{
    OwnedUserId,
    api::{auth_scheme::AccessToken, metadata, request, response},
};

metadata! {
    method: POST,
    rate_limited: false,
    authentication: AccessToken,
    path: "/_synapse/admin/v1/deactivate/{user_id}",
}

#[request]
pub struct Request {
    /// User ID
    #[ruma_api(path)]
    pub user_id: OwnedUserId,

    /// Flag whether to erase the account.
    #[serde(default, skip_serializing_if = "ruma::serde::is_default")]
    pub erase: bool,
}

#[derive(Default)]
#[response]
pub struct Response {}

impl Request {
    /// Creates a `Request` with the given user ID.
    pub fn new(user_id: OwnedUserId) -> Self {
        Self { user_id, erase: false }
    }
}

impl Response {
    /// Creates an empty `Response`.
    pub fn new() -> Self {
        Self {}
    }
}

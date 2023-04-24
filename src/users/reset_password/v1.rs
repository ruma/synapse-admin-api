//! [POST /_synapse/admin/v1/reset_password/:user_id](https://github.com/matrix-org/synapse/blob/master/docs/admin_api/user_admin_api.rst#reset-password)

use ruma::{
    api::{metadata, request, response, Metadata},
    OwnedUserId,
};

const METADATA: Metadata = metadata! {
    method: POST,
    rate_limited: false,
    authentication: AccessToken,
    history: {
        unstable => "/_synapse/admin/v1/reset_password/:user_id",
    }
};

#[request]
pub struct Request {
    /// user ID of the account to reset the password
    #[ruma_api(path)]
    pub user_id: OwnedUserId,

    /// new password
    pub new_password: String,

    /// Logout all devices of the user, so it necessary to login with the new password again.
    /// Defaults to true.
    #[serde(default = "ruma::serde::default_true")]
    pub logout_devices: bool,
}

#[derive(Default)]
#[response]
pub struct Response {}

impl Request {
    /// Creates an `Request` with the given user ID and the new password.
    pub fn new(user_id: OwnedUserId, new_password: String) -> Self {
        Self { user_id, new_password, logout_devices: true }
    }
}

impl Response {
    /// Creates an empty `Response`.
    pub fn new() -> Self {
        Self {}
    }
}

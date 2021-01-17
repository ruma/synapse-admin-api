//! [POST /_synapse/admin/v1/reset_password/:user_id](https://github.com/matrix-org/synapse/blob/master/docs/admin_api/user_admin_api.rst#reset-password)

use ruma::{api::ruma_api, identifiers::UserId};

ruma_api! {
    metadata: {
        description: "password reset endpoint",
        method: POST,
        name: "reset_password",
        path: "/_synapse/admin/v1/reset_password/:user_id",
        rate_limited: false,
        authentication: AccessToken,
    }

    request: {
        /// user ID of the account to reset the password
        #[ruma_api(path)]
        pub user_id: &'a UserId,

        /// new password
        pub new_password: &'a str,

        /// Logout all devices of the user, so it necessary to login with the new password again.
        /// Defaults to true.
        #[serde(default = "ruma::serde::default_true")]
        pub logout_devices: bool,
    }

    #[derive(Default)]
    response: {}
}

impl<'a> Request<'a> {
    /// Creates an `Request` with the given user ID and the new password.
    pub fn new(user_id: &'a UserId, new_password: &'a str) -> Self {
        Self { user_id, new_password, logout_devices: true }
    }
}

impl Response {
    /// Creates an empty `Response`.
    pub fn new() -> Self {
        Self
    }
}

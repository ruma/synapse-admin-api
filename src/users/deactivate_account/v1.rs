//! [POST /_synapse/admin/v1/deactivate/:user_id](https://github.com/matrix-org/synapse/blob/develop/docs/admin_api/user_admin_api.rst#deactivate-account)

use ruma::{api::ruma_api, UserId};

ruma_api! {
    metadata: {
        description: "Deactivate a user account",
        method: POST,
        name: "deactivate_account_v1",
        unstable_path: "/_synapse/admin/v1/deactivate/:user_id",
        rate_limited: false,
        authentication: AccessToken,
    }

    request: {
        /// User ID
        #[ruma_api(path)]
        pub user_id: &'a UserId,

        /// Flag wether to erase the account.
        #[serde(default = "ruma::serde::default_false", skip_serializing_if = "ruma::serde::is_false")]
        pub erase: bool,
    }

    #[derive(Default)]
    response: {}
}

impl<'a> Request<'a> {
    /// Creates a `Request` with the given user ID.
    pub fn new(user_id: &'a UserId) -> Self {
        Self { user_id }
    }
}

impl Response {
    /// Creates an empty `Response`.
    pub fn new() -> Self {
        Self
    }
}

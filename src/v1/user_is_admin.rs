//! [GET /_synapse/admin/v1/users/:user_id/admin](https://github.com/matrix-org/synapse/blob/master/docs/admin_api/user_admin_api.rst#get-whether-a-user-is-a-server-administrator-or-not)
use ruma::{api::ruma_api, identifiers::UserId};

ruma_api! {
    metadata: {
        description: "is admin endpoint",
        method: GET,
        name: "user_is_admin",
        path: "/_synapse/admin/v1/users/:user_id/admin",
        rate_limited: false,
        authentication: AccessToken,
    }

    request: {
        /// User to check.
        #[ruma_api(path)]
        pub user_id: &'a UserId,
    }

    response: {
        /// Whether the requested user ID is an admin.
        pub admin: bool,
    }

}

impl<'a> Request<'a> {
    /// Creates an `Request` with the given user ID.
    pub fn new(user_id: &'a UserId) -> Self {
        Self { user_id }
    }
}

impl Response {
    /// Creates a `Response` with the given admin flag.
    pub fn new(admin: bool) -> Self {
        Self { admin }
    }
}

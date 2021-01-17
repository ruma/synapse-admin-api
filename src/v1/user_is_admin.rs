//! [GET /_synapse/admin/v1/users/:user_id/admin](https://github.com/matrix-org/synapse/blob/master/docs/admin_api/user_admin_api.rst#get-whether-a-user-is-a-server-administrator-or-not)
use ruma::api::ruma_api;
use ruma::identifiers::UserId;

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

        /// user ID for the account
        #[ruma_api(path)]
        pub user_id: &'a UserId,
    }

    response: {
        /// Shows if the requested user ID is an admin?
        pub admin: bool,
    }

}

impl<'a> Request<'a> {
    /// Creates an `Request` with the given user ID.
    pub fn new(user_id: &'a UserId) -> Self {
        Self { user_id }
    }
}
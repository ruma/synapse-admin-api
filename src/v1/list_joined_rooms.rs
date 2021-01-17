//! [GET /_synapse/admin/v1/users/:user_id/joined_rooms](https://github.com/matrix-org/synapse/blob/master/docs/admin_api/user_admin_api.rst#list-room-memberships-of-an-user)

use js_int::UInt;

use ruma::api::ruma_api;
use ruma::identifiers::UserId;

ruma_api! {
    metadata: {
        description: "list room memberships of a user",
        method: GET,
        name: "query_user",
        path: "/_synapse/admin/v1/users/:user_id/joined_rooms",
        rate_limited: false,
        authentication: AccessToken,
    }

    request: {
        /// user ID for the account
        #[ruma_api(path)]
        pub user_id: &'a UserId,
    }

    response: {
        /// List all joined roo IDs
        pub joined_rooms: Vec<ruma::RoomId>,
        /// amount of joined of rooms
        pub total: UInt,
    }

}

impl<'a> Request<'a> {
    /// Creates an `Request` with the given user ID.
    pub fn new(user_id: &'a UserId) -> Self {
        Self { user_id }
    }
}

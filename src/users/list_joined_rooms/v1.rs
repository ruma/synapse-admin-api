//! [GET /_synapse/admin/v1/users/:user_id/joined_rooms](https://github.com/matrix-org/synapse/blob/master/docs/admin_api/user_admin_api.rst#list-room-memberships-of-an-user)

use ruma::{api::ruma_api, RoomId, UInt, UserId};

ruma_api! {
    metadata: {
        description: "list room memberships of a user",
        method: GET,
        name: "list_joined_rooms_v1",
        path: "/_synapse/admin/v1/users/:user_id/joined_rooms",
        rate_limited: false,
        authentication: AccessToken,
    }

    request: {
        /// User ID
        #[ruma_api(path)]
        pub user_id: &'a UserId,
    }

    response: {
        /// List all joined roo IDs.
        pub joined_rooms: Vec<RoomId>,
        /// Amount of joined of rooms.
        pub total: UInt,
    }

}

impl<'a> Request<'a> {
    /// Creates an `Request` with the given user ID.
    pub fn new(user_id: &'a UserId) -> Self {
        Self { user_id }
    }
}

impl Response {
    /// Creates a `Response` with the given joined rooms and the total amount of them.
    pub fn new(joined_rooms: Vec<RoomId>, total: UInt) -> Self {
        Self { joined_rooms, total }
    }
}

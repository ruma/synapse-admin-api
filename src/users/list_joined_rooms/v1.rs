//! [GET /_synapse/admin/v1/users/:user_id/joined_rooms](https://github.com/element-hq/synapse/blob/master/docs/admin_api/user_admin_api.md#list-joined-rooms-of-a-user)

use ruma::{
    OwnedRoomId, OwnedUserId, UInt,
    api::{auth_scheme::AccessToken, metadata, request, response},
};

metadata! {
    method: GET,
    rate_limited: false,
    authentication: AccessToken,
    path: "/_synapse/admin/v1/users/{user_id}/joined_rooms",
}

#[request]
pub struct Request {
    /// User ID
    #[ruma_api(path)]
    pub user_id: OwnedUserId,
}

#[response]
pub struct Response {
    /// List all joined roo IDs.
    pub joined_rooms: Vec<OwnedRoomId>,

    /// Amount of joined of rooms.
    pub total: UInt,
}

impl Request {
    /// Creates an `Request` with the given user ID.
    pub fn new(user_id: OwnedUserId) -> Self {
        Self { user_id }
    }
}

impl Response {
    /// Creates a `Response` with the given joined rooms and the total amount of them.
    pub fn new(joined_rooms: Vec<OwnedRoomId>, total: UInt) -> Self {
        Self { joined_rooms, total }
    }
}

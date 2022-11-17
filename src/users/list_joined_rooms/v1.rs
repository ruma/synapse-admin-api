//! [GET /_synapse/admin/v1/users/:user_id/joined_rooms](https://github.com/matrix-org/synapse/blob/master/docs/admin_api/user_admin_api.rst#list-room-memberships-of-an-user)

use ruma::{
    api::{metadata, request, response, Metadata},
    OwnedRoomId, UInt, UserId,
};

const METADATA: Metadata = metadata! {
    method: GET,
    rate_limited: false,
    authentication: AccessToken,
    history: {
        unstable => "/_synapse/admin/v1/users/:user_id/joined_rooms",
    }
};

#[request]
pub struct Request<'a> {
    /// User ID
    #[ruma_api(path)]
    pub user_id: &'a UserId,
}

#[response]
pub struct Response {
    /// List all joined roo IDs.
    pub joined_rooms: Vec<OwnedRoomId>,

    /// Amount of joined of rooms.
    pub total: UInt,
}

impl<'a> Request<'a> {
    /// Creates an `Request` with the given user ID.
    pub fn new(user_id: &'a UserId) -> Self {
        Self { user_id }
    }
}

impl Response {
    /// Creates a `Response` with the given joined rooms and the total amount of them.
    pub fn new(joined_rooms: Vec<OwnedRoomId>, total: UInt) -> Self {
        Self { joined_rooms, total }
    }
}

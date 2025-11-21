//! [GET /_synapse/admin/v1/rooms/:room_id/members](https://github.com/element-hq/synapse/blob/master/docs/admin_api/rooms.md#room-members-api)
use ruma::{
    OwnedRoomId, OwnedUserId, UInt,
    api::{auth_scheme::AccessToken, metadata, request, response},
};

metadata! {
    method: GET,
    rate_limited: false,
    authentication: AccessToken,
    path: "/_synapse/admin/v1/rooms/{room_id}/members",
}

#[request]
pub struct Request {
    /// ID of the room to list the members of.
    #[ruma_api(path)]
    pub room_id: OwnedRoomId,
}

#[response]
pub struct Response {
    /// List of members that are present in the room
    pub members: Vec<OwnedUserId>,

    /// Amount of members in the room.
    pub total: UInt,
}

impl Request {
    /// Creates a `Request` with the given room ID.
    pub fn new(room_id: OwnedRoomId) -> Self {
        Self { room_id }
    }
}

impl Response {
    /// Creates a `Response` with the given members and total count,
    pub fn new(members: Vec<OwnedUserId>, total: UInt) -> Self {
        Self { members, total }
    }
}

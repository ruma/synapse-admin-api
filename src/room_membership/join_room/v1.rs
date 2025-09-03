//! [POST /_synapse/admin/v1/join/:room_id_or_alias](https://github.com/element-hq/synapse/blob/master/docs/admin_api/room_membership.md)

use ruma::{
    api::{request, response, Metadata},
    metadata, OwnedRoomId, OwnedRoomOrAliasId, OwnedUserId,
};

const METADATA: Metadata = metadata! {
    method: POST,
    rate_limited: false,
    authentication: AccessToken,
    history: {
        unstable => "/_synapse/admin/v1/join/{room_id_or_alias}",
    }
};

#[request]
pub struct Request {
    /// Alias or ID of the room to join.
    #[ruma_api(path)]
    pub room_id_or_alias: OwnedRoomOrAliasId,

    /// User to join the room.
    pub user_id: OwnedUserId,
}

#[response]
pub struct Response {
    /// Room ID of the joined room.
    pub room_id: OwnedRoomId,
}

impl Request {
    /// Creates a new `Request` with the given room or alias ID and user id.
    pub fn new(room_id_or_alias: OwnedRoomOrAliasId, user_id: OwnedUserId) -> Self {
        Self { room_id_or_alias, user_id }
    }
}

impl Response {
    /// Creates a new `Response` with the given room id
    pub fn new(room_id: OwnedRoomId) -> Self {
        Self { room_id }
    }
}

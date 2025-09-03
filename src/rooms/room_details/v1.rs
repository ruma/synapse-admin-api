//! [GET /_synapse/admin/v1/rooms/:room_id](https://github.com/element-hq/synapse/blob/master/docs/admin_api/rooms.md#room-details-api)
use ruma::{
    api::{metadata, request, response, Metadata},
    events::room::{guest_access::GuestAccess, history_visibility::HistoryVisibility},
    room::{JoinRuleKind, RoomType},
    uint, OwnedMxcUri, OwnedRoomAliasId, OwnedRoomId, OwnedUserId, UInt,
};

const METADATA: Metadata = metadata! {
    method: GET,
    rate_limited: false,
    authentication: AccessToken,
    history: {
        unstable => "/_synapse/admin/v1/rooms/{room_id}",
    }
};

#[request]
pub struct Request {
    /// ID of the room to show the details of.
    #[ruma_api(path)]
    pub room_id: OwnedRoomId,
}

#[response]
pub struct Response {
    /// Room ID
    pub room_id: OwnedRoomId,

    /// Room name
    pub name: Option<String>,

    /// Room topic
    pub topic: Option<String>,

    /// Room avatar
    pub avatar: Option<OwnedMxcUri>,

    /// Room alias ID
    pub canonical_alias: Option<OwnedRoomAliasId>,

    /// Amount of joined members.
    pub joined_members: UInt,

    /// Amount of local members.
    pub joined_local_members: UInt,

    /// Amount of local devices.
    pub joined_local_devices: UInt,

    /// Room version
    pub version: Option<String>,

    /// User ID of the room creator.
    #[serde(deserialize_with = "ruma::serde::empty_string_as_none")]
    pub creator: Option<OwnedUserId>,

    /// Room encryption.
    pub encryption: Option<String>,

    /// Whether the room is federatable
    #[serde(deserialize_with = "crate::serde::bool_or_uint")]
    pub federatable: bool,

    /// Whether the room is public.
    #[serde(deserialize_with = "crate::serde::bool_or_uint")]
    pub public: bool,

    /// Join rules of the room.
    pub join_rules: Option<JoinRuleKind>,

    /// Guest access of the room
    pub guest_access: Option<GuestAccess>,

    /// History visibility of the room
    pub history_visibility: Option<HistoryVisibility>,

    /// State events of the room.
    pub state_events: UInt,

    /// Room type of the room.
    pub room_type: Option<RoomType>,

    /// Whether all local users have forgotten the room.
    #[serde(deserialize_with = "crate::serde::bool_or_uint")]
    pub forgotten: bool,
}

impl Request {
    /// Creates a `Request` with the given room ID.
    pub fn new(room_id: OwnedRoomId) -> Self {
        Self { room_id }
    }
}

impl Response {
    /// Creates a `Response` with the given room ID and default values.
    pub fn new(room_id: OwnedRoomId) -> Self {
        Self {
            room_id,
            name: None,
            topic: None,
            avatar: None,
            canonical_alias: None,
            joined_members: uint!(0),
            joined_local_members: uint!(0),
            joined_local_devices: uint!(0),
            version: None,
            creator: None,
            encryption: None,
            federatable: false,
            public: false,
            join_rules: None,
            guest_access: None,
            history_visibility: None,
            state_events: uint!(0),
            room_type: None,
            forgotten: false,
        }
    }
}

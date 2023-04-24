//! [GET /_synapse/admin/v1/rooms](https://github.com/matrix-org/synapse/blob/master/docs/admin_api/rooms.md#list-room-api)
use ruma::{
    api::{metadata, request, response, Metadata},
    events::room::{
        guest_access::GuestAccess, history_visibility::HistoryVisibility, join_rules::JoinRule,
    },
    serde::StringEnum,
    OwnedRoomAliasId, OwnedRoomId, OwnedUserId, UInt,
};
use serde::{Deserialize, Serialize};

const METADATA: Metadata = metadata! {
    method: GET,
    rate_limited: false,
    authentication: AccessToken,
    history: {
        unstable => "/_synapse/admin/v1/rooms",
    }
};

#[request]
#[derive(Default)]
pub struct Request {
    /// Offset in the returned list. Defaults to 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub from: Option<UInt>,

    /// Maximum amount of rooms to return. Defaults to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub limit: Option<UInt>,

    /// Sort order of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub order_by: Option<RoomSortOrder>,

    /// Sort direction of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub dir: Option<SortDirection>,

    /// Filter rooms by their room name. Search term can be contained in any part of the room name.
    /// Defaults to no filtering.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub search_term: Option<String>,
}

#[derive(Default)]
#[response]
pub struct Response {
    /// List all RoomDetails.
    pub rooms: Vec<RoomDetails>,

    /// Offset.
    pub offset: UInt,

    /// Total amount of rooms.
    pub total_rooms: UInt,

    /// Token to receive the next RoomDetails batch.
    pub next_batch: Option<UInt>,

    /// Token to receive the previous RoomDetails batch.
    pub prev_batch: Option<UInt>,
}

impl Request {
    /// Creates an empty `Request`.
    pub fn new() -> Self {
        Default::default()
    }
}

impl Response {
    /// Creates an empty `Response`.
    pub fn new() -> Self {
        Default::default()
    }
}

/// Enum to define the sorting method of rooms.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, StringEnum)]
#[ruma_enum(rename_all = "snake_case")]
pub enum RoomSortOrder {
    /// Sort by name alphabetical
    Name,

    /// Sort by canonical alias
    CanonicalAlias,

    /// Sort by joined members
    JoinedMembers,

    /// Sort by joined local members
    JoinedLocalMembers,

    /// Sort by version
    Version,

    /// Sort by creator
    Creator,

    /// Sort by encryption
    Encryption,

    /// Sort by feaeratable
    Federatable,

    /// Sort by public
    Public,

    /// Sort by join rules
    JoinRules,

    /// Sort by guest access
    GuestAccess,

    /// Sort by history visibility
    HistoryVisibility,

    /// Sort by state events
    StateEvents,

    #[doc(hidden)]
    _Custom(crate::PrivOwnedStr),
}

/// Enum to define the sort order direction.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, StringEnum)]
pub enum SortDirection {
    /// Sort direction backward.
    #[ruma_enum(rename = "b")]
    Backward,

    /// Sort direction forward.
    #[ruma_enum(rename = "f")]
    Forward,

    #[doc(hidden)]
    _Custom(crate::PrivOwnedStr),
}

/// Structure for all the room details.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RoomDetails {
    /// Room ID
    pub room_id: OwnedRoomId,

    /// Room name
    pub name: Option<String>,

    /// Room alias ID
    pub canonical_alias: Option<OwnedRoomAliasId>,

    /// Amount of joined members.
    pub joined_members: UInt,

    /// Amount of local members.
    pub joined_local_members: UInt,

    /// Room version
    pub version: String,

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
    pub join_rules: JoinRule,

    /// Guest access of the room
    pub guest_access: GuestAccess,

    /// History visibility of the room
    pub history_visibility: HistoryVisibility,

    /// State events of the room.
    pub state_events: UInt,
}

//! [GET /_synapse/admin/v1/background_updates/status](https://github.com/element-hq/synapse/blob/master/docs/usage/administration/admin_api/background_updates.md#status)

use std::collections::HashMap;

use ruma::api::{metadata, request, response, Metadata};
use serde::{Deserialize, Serialize};

const METADATA: Metadata = metadata! {
    method: GET,
    rate_limited: false,
    authentication: AccessToken,
    history: {
        unstable => "/_synapse/admin/v1/background_updates/status",
    }
};

#[request]
#[derive(Default)]
pub struct Request {}

#[response]
#[derive(Serialize, Deserialize, PartialEq)]
pub struct Response {
    /// Whether the background updates are enabled.
    pub enabled: bool,

    /// The current update based on database name.
    pub current_updates: HashMap<String, CurrentUpdate>,
}

/// Information about a current update.
///
/// To create an instance of this type, first create a `CurrentUpdateInit` and convert it via
/// `CurrentUpdate::from` / `.into()`.
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[cfg_attr(not(ruma_unstable_exhaustive_types), non_exhaustive)]
pub struct CurrentUpdate {
    /// Name of the update.
    pub name: String,

    /// Total number of processed "items".
    pub total_item_count: u64,

    /// Runtime of background process, not including sleeping time.
    pub total_duration_ms: f64,

    /// Items processed per millisecond based on an exponential average.
    pub average_items_per_ms: f64,
}

/// Initial set of fields of [`CurrentUpdate`].
///
/// This struct will not be updated even if additional fields are added to `CurrentUpdate`.
#[derive(Debug)]
#[allow(clippy::exhaustive_structs)]
pub struct CurrentUpdateInit {
    /// Name of the update.
    pub name: String,

    /// Total number of processed "items".
    pub total_item_count: u64,

    /// Runtime of background process, not including sleeping time.
    pub total_duration_ms: f64,

    /// Items processed per millisecond based on an exponential average.
    pub average_items_per_ms: f64,
}

impl From<CurrentUpdateInit> for CurrentUpdate {
    fn from(value: CurrentUpdateInit) -> Self {
        let CurrentUpdateInit { name, total_item_count, total_duration_ms, average_items_per_ms } =
            value;
        Self { name, total_item_count, total_duration_ms, average_items_per_ms }
    }
}

impl Request {
    /// Creates an empty `Request`.
    pub fn new() -> Self {
        Self {}
    }
}

impl Response {
    /// Creates a `Response` with the given parameters.
    pub fn new(enabled: bool, current_updates: HashMap<String, CurrentUpdate>) -> Self {
        Self { enabled, current_updates }
    }
}

#[test]
fn test_status_background_updates() {
    let name = "current update 1";
    let total_item_count = 123_456_789;
    let total_duration_ms = 2_134_567.123_45;
    let average_items_per_ms = 2.5;

    // Create the current update
    let update = CurrentUpdate {
        name: name.to_owned(),
        total_item_count,
        total_duration_ms,
        average_items_per_ms,
    };
    assert_eq!(update.name, name);
    assert_eq!(update.total_item_count, total_item_count);
    assert_eq!(update.total_duration_ms, total_duration_ms);
    assert_eq!(update.average_items_per_ms, average_items_per_ms);

    // Create the hashmap
    let mut current_updates = HashMap::new();
    current_updates.insert("master".to_owned(), update);
    let enabled = true;

    let response = Response::new(enabled, current_updates);

    // Serialize
    let serialized = serde_json::to_string(&response).expect("Failed to serialize");
    assert_eq!(serialized, "{\"enabled\":true,\"current_updates\":{\"master\":{\"name\":\"current update 1\",\"total_item_count\":123456789,\"total_duration_ms\":2134567.12345,\"average_items_per_ms\":2.5}}}");

    // Deserialize
    let deserialized: Response = serde_json::from_str(&serialized).expect("Failed to deserialize");
    assert_eq!(deserialized, response);
}

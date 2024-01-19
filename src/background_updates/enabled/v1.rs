//! [POST /_synapse/admin/v1/background_updates/enabled](https://github.com/element-hq/synapse/blob/develop/docs/usage/administration/admin_api/background_updates.md#enabled)

use ruma::api::{Metadata, request, response};
use ruma::metadata;
use serde::{Deserialize, Serialize};

const METADATA: Metadata = metadata! {
    method: POST,
    rate_limited: false,
    authentication: AccessToken,
    history: {
        unstable => "/_synapse/admin/v1/background_updates/enabled",
    }
};

#[request]
pub struct Request {
    /// Sets whether the background updates are enabled or disabled
    pub enabled: bool
}

#[response]
#[derive(Serialize, Deserialize, PartialEq)]
pub struct Response {
    /// Whether the background updates are enabled or disabled.
    pub enabled: bool,
}

impl Request {
    /// Creates a `Request` with the given `enabled` value.
    pub fn new(enabled: bool) -> Self { Self { enabled } }
}

impl Response {
    /// Creates a `Response` with the given `enabled` value.
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}

#[test]
fn test_enabled_background_updates() {
   let enabled = true;

    // Check create request
    let request = Request::new(enabled);
    assert_eq!(request.enabled, true);

    // Check create response
    let response = Response::new(enabled);
    assert_eq!(request.enabled, true);

    // Serialize
    let serialized = serde_json::to_string(&response).expect("Failed to serialize");
    assert_eq!(serialized, "{\"enabled\":true}");

    // Deserialize
    let deserialized: Response = serde_json::from_str(&serialized).expect("Failed to deserialize");
    assert_eq!(deserialized, response);
    assert_eq!(deserialized.enabled, true);
}

//! [PUT /_synapse/admin/v1/experimental_features/:user_id](https://github.com/element-hq/synapse/blob/master/docs/admin_api/experimental_features.md#enablingdisabling-features)

use ruma::{
    api::{request, response, Metadata},
    metadata, OwnedUserId,
};
use serde::{Deserialize, Serialize};

const METADATA: Metadata = metadata! {
    method: PUT,
    rate_limited: false,
    authentication: AccessToken,
    history: {
        unstable => "/_synapse/admin/v1/experimental_features/{user_id}",
    }
};

#[request]
#[derive(Serialize, Deserialize, PartialEq)]
pub struct Request {
    /// User ID.
    #[ruma_api(path)]
    pub user_id: OwnedUserId,
    /// Experimental features per user.
    pub features: ExperimentalFeatures,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Default)]
#[cfg_attr(not(ruma_unstable_exhaustive_types), non_exhaustive)]
pub struct ExperimentalFeatures {
    /// Whether busy presence state is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msc3026: Option<bool>,
    /// Whether remotely toggling push notifications for another client is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msc3881: Option<bool>,
    /// Do not require UIA when first uploading cross-signing keys.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msc3967: Option<bool>,
}

impl ExperimentalFeatures {
    /// Construct an empty `ExperimentalFeatures`.
    pub fn new() -> Self {
        Self::default()
    }
}

#[response]
#[derive(Default)]
pub struct Response {}

impl Request {
    /// Creates a `Request` with the given user ID and experimental features to enable or
    /// disable for a user.
    pub fn new(user_id: OwnedUserId, features: ExperimentalFeatures) -> Self {
        Self { user_id, features }
    }
}

impl Response {
    /// Creates an empty `Response`.
    pub fn new() -> Self {
        Self {}
    }
}

#[test]
fn test_enable_features() {
    use std::convert::TryFrom;

    use ruma::UserId;

    let features =
        ExperimentalFeatures { msc3026: Option::from(true), msc3881: None, msc3967: None };

    let user_id: &UserId =
        <&UserId>::try_from("@carl:example.com").expect("Failed to create UserId.");

    // Check create request
    let request = Request::new(user_id.to_owned(), features);

    // Serialize
    let serialized = serde_json::to_string(&request).expect("Failed to serialize");
    assert_eq!(serialized, "{\"user_id\":\"@carl:example.com\",\"features\":{\"msc3026\":true}}");

    // Deserialize
    let deserialized: Request = serde_json::from_str(&serialized).expect("Failed to deserialize");
    assert_eq!(deserialized, request);
}

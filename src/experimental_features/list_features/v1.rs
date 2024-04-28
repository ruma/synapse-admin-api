//! [GET /_synapse/admin/v1/experimental_features/:user_id](https://github.com/element-hq/synapse/blob/develop/docs/admin_api/experimental_features.md#listing-enabled-features)

use crate::experimental_features::enable_features::v1::ExperimentalFeatures;
use ruma::{
    api::{request, response, Metadata},
    metadata, OwnedUserId,
};
use serde::{Deserialize, Serialize};

const METADATA: Metadata = metadata! {
    method: GET,
    rate_limited: false,
    authentication: AccessToken,
    history: {
        unstable => "/_synapse/admin/v1/experimental_features/:user_id",
    }
};

#[request]
#[derive(Serialize, Deserialize, PartialEq)]
pub struct Request {
    /// User ID.
    #[ruma_api(path)]
    pub user_id: OwnedUserId,
}

#[response]
#[derive(Serialize, Deserialize, PartialEq)]
pub struct Response {
    /// Experimental features per user.
    pub features: ExperimentalFeatures,
}

impl Request {
    /// Creates a `Request` with the given user ID.
    pub fn new(user_id: OwnedUserId) -> Self {
        Self { user_id }
    }
}

impl Response {
    /// Creates a `Response` with experimental features for the user.
    pub fn new(features: ExperimentalFeatures) -> Self {
        Self { features }
    }
}

#[test]
fn test_list_features() {
    let response = Response {
        features: ExperimentalFeatures {
            msc3026: Option::from(false),
            msc3881: None,
            msc3967: None,
        },
    };

    let json_str = "{\"features\":{\"msc3026\":false}}";
    // Deserialize
    let deserialized: Response = serde_json::from_str(json_str).expect("Failed to deserialize");
    assert_eq!(deserialized, response);

    // Serialize
    let serialized = serde_json::to_string(&response).expect("Failed to serialize");
    assert_eq!(serialized, json_str);
}

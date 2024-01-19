//! [GET /_synapse/admin/v1/server_version](https://github.com/element-hq/synapse/blob/master/docs/admin_api/version_api.md)

use ruma::api::{metadata, request, response, Metadata};
use serde::{Deserialize, Serialize};

const METADATA: Metadata = metadata! {
    method: GET,
    rate_limited: false,
    authentication: None, // AccessToken?
    history: {
        unstable => "/_synapse/admin/v1/server_version",
    }
};

#[request]
#[derive(Default)]
pub struct Request {}

#[response]
#[derive(Serialize, Deserialize)]
pub struct Response {
    /// The Synapse version.
    pub server_version: String,

    /// The Python version.
    /// Optional since Synapse 1.94.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub python_version: Option<String>,
}

impl Request {
    /// Creates an empty `Request`.
    pub fn new() -> Self {
        Self {}
    }
}

impl Response {
    /// Creates a `Response` with the given Synapse and Python versions.
    pub fn new(server_version: String, python_version: Option<String>) -> Self {
        Self { server_version, python_version }
    }
}

#[test]
fn test_response_with_python_version() {
    use serde_json;

    let server_version = "1.2.3".to_string();
    let python_version = Some("4.5.6".to_string());

    // create response case
    let response = Response::new(server_version.clone(), python_version.clone());
    assert_eq!(response.server_version, server_version);
    assert_eq!(response.python_version, python_version);

    // serialization case
    let serialized = serde_json::to_string(&response).unwrap();
    assert_eq!(serialized, "{\"server_version\":\"1.2.3\",\"python_version\":\"4.5.6\"}");

    // deserialization case
    let deserialized: Response = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized.server_version, "1.2.3");
    assert_eq!(deserialized.python_version, Some("4.5.6".to_string()));
}

#[test]
fn test_response_without_python_version() {
    use serde_json;

    let server_version = "1.2.3".to_string();
    let python_version = None;

    // create response case
    let response = Response::new(server_version.clone(), python_version);
    assert_eq!(response.server_version, server_version);
    assert!(response.python_version.is_none());

    // serialization case
    let serialized = serde_json::to_string(&response).unwrap();
    assert_eq!(serialized, "{\"server_version\":\"1.2.3\"}");

    // deserialization case
    let deserialized: Response = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized.server_version, "1.2.3");
    assert!(deserialized.python_version.is_none());
}

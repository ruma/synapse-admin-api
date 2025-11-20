//! [GET /_synapse/admin/v1/server_version](https://github.com/element-hq/synapse/blob/master/docs/admin_api/version_api.md)

use ruma::api::{auth_scheme::NoAuthentication, metadata, request, response};
use serde::{Deserialize, Serialize};

metadata! {
    method: GET,
    rate_limited: false,
    authentication: NoAuthentication, // AccessToken?
    path: "/_synapse/admin/v1/server_version",
}

#[request]
#[derive(Default)]
pub struct Request {}

#[response]
#[derive(Serialize, Deserialize)]
pub struct Response {
    /// The Synapse version.
    pub server_version: String,

    /// The Python version.
    ///
    /// Only sent by Synapse versions before 1.94.0.
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
    /// Creates a `Response` with the given Synapse versions.
    pub fn new(server_version: String) -> Self {
        Self { server_version, python_version: None }
    }
}

#[test]
fn test_response_with_python_version() {
    use serde_json;

    let server_version = "1.2.3";

    // Check create response case
    let response = Response::new(server_version.to_owned());
    assert_eq!(response.server_version, server_version);
    assert_eq!(response.python_version, None);

    // Check serialization case
    let serialized = serde_json::to_string(&response).unwrap();
    assert_eq!(serialized, "{\"server_version\":\"1.2.3\"}");

    // Check deserialization case
    let deserialized: Response = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized.server_version, "1.2.3");
    assert_eq!(deserialized.python_version, None);

    // Check backwards compatibility
    let old_serialized = "{\"server_version\":\"1.2.3\",\"python_version\":\"4.5.6\"}";
    let old_deserialized: Response = serde_json::from_str(old_serialized).unwrap();
    assert_eq!(old_deserialized.server_version, "1.2.3");
    assert_eq!(old_deserialized.python_version.as_deref(), Some("4.5.6"));
}

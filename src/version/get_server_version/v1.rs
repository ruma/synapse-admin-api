//! [GET /_synapse/admin/v1/server_version](https://github.com/matrix-org/synapse/blob/master/docs/admin_api/version_api.rst)

use ruma::api::{metadata, request, response, Metadata};

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
pub struct Response {
    /// The Synapse version.
    pub server_version: String,

    /// The Python version.
    pub python_version: String,
}

impl Request {
    /// Creates an empty `Request`.
    pub fn new() -> Self {
        Self {}
    }
}

impl Response {
    /// Creates a `Response` with the given Synapse and Python versions.
    pub fn new(server_version: String, python_version: String) -> Self {
        Self { server_version, python_version }
    }
}

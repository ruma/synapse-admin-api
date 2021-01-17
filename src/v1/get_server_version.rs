//! [GET /_synapse/admin/v1/server_version](https://github.com/matrix-org/synapse/blob/master/docs/admin_api/version_api.rst)

use ruma::api::ruma_api;

ruma_api! {
    metadata: {
        description: "Get the Synapse and Python version of this homeserver.",
        method: GET,
        name: "get_server_version",
        path: "/_synapse/admin/v1/server_version",
        rate_limited: false,
        authentication: None, // AccessToken?
    }

    #[derive(Default)]
    request: {}

    response: {
        /// The Synapse version.
        pub server_version: String,

        /// The Python version.
        pub python_version: String,
    }
}

impl Request {
    /// Creates an empty `Request`.
    pub fn new() -> Self {
        Self
    }
}

impl Response {
    /// Creates a `Response` with the given Synapse and Python versions.
    pub fn new(server_version: String, python_version: String) -> Self {
        Self { server_version, python_version }
    }
}

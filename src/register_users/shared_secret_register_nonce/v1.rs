//! [GET /_synapse/admin/v1/register](https://matrix-org.github.io/synapse/latest/admin_api/register_api.html#shared-secret-registration)

use ruma::api::ruma_api;
use serde::{Deserialize, Serialize};

ruma_api! {
    metadata: {
        description: "Get shared-secret nonce",
        method: GET,
        name: "shared_secret_register_nonce_v1",
        unstable_path: "/_synapse/admin/v1/register",
        rate_limited: false,
        authentication: None,
    }

    #[derive(Default)]
    request: {}

    response: {
        /// The nonce.
        #[ruma_api(body)]
        pub nonce: Nonce,
    }
}

impl Request {
    /// Creates an empty `Request`.
    pub fn new() -> Self {
        Default::default()
    }
}

impl Response {
    /// Creates a `Response` with the given `nonce`.
    pub fn new(nonce: String) -> Self {
        Self { nonce: Nonce { nonce } }
    }
}

/// Shared-secret registration nonce.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Nonce {
    /// The nonce that can be used for shared-secret registration.
    pub nonce: String,
}

//! [GET /_synapse/admin/v1/register](https://github.com/element-hq/synapse/blob/master/docs/admin_api/register_api.md)

use ruma::api::{metadata, request, response, Metadata};

const METADATA: Metadata = metadata! {
    method: GET,
    rate_limited: false,
    authentication: None,
    history: {
        unstable => "/_synapse/admin/v1/register",
    }
};

#[request]
#[derive(Default)]
pub struct Request {}

#[response]
pub struct Response {
    /// The nonce that can be used for shared-secret registration.
    pub nonce: String,
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
        Self { nonce }
    }
}

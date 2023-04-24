//! [GET /_synapse/admin/v1/account_validity/validity](https://github.com/matrix-org/synapse/blob/master/docs/admin_api/account_validity.rst)

use ruma::{
    api::{metadata, request, response, Metadata},
    MilliSecondsSinceUnixEpoch, OwnedUserId,
};

const METADATA: Metadata = metadata! {
    method: POST,
    rate_limited: false,
    authentication: AccessToken,
    history: {
        unstable => "/_synapse/admin/v1/account_validity/validity",
    }
};

#[request]
pub struct Request {
    /// user ID for the account to renew
    pub user_id: OwnedUserId,

    /// This is an optional parameter, it overrides the expiration date,
    /// which otherwise defaults to now + validity period configured at the server
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_ts: Option<MilliSecondsSinceUnixEpoch>,

    /// This is an optional parameter, it enables/disables sending renewal emails to the user.
    /// Defaults to true.
    #[serde(default = "ruma::serde::default_true", skip_serializing_if = "ruma::serde::is_true")]
    pub enable_renewal_emails: bool,
}

#[response]
pub struct Response {
    /// The new expiration date for this account, as a timestamp in milliseconds since epoch
    pub expiration_ts: MilliSecondsSinceUnixEpoch,
}

impl Request {
    /// Creates an `Request` with the given user ID.
    pub fn new(user_id: OwnedUserId) -> Self {
        Self { user_id, expiration_ts: None, enable_renewal_emails: true }
    }
}

impl Response {
    /// Creates a `Response` with the new expiration date for this account,
    pub fn new(expiration_ts: MilliSecondsSinceUnixEpoch) -> Self {
        Self { expiration_ts }
    }
}

//! [GET /_synapse/admin/v1/account_validity/validity](https://github.com/matrix-org/synapse/blob/master/docs/admin_api/account_validity.rst)

use std::time::SystemTime;

use ruma::api::ruma_api;
use ruma::identifiers::UserId;

ruma_api! {
    metadata: {
        description: "Extend the account validity.",
        method: POST,
        name: "set_account_validity",
        path: "/_synapse/admin/v1/account_validity/validity",
        rate_limited: false,
        authentication: None, // AccessToken?
    }

    request: {
        /// user ID for the account to renew
        pub user_id: &'a UserId,

        /// This is an optional parameter, it overrides the expiration date,
        /// which otherwise defaults to now + validity period configured at the server
        #[serde(
            with = "ruma_serde::time::opt_ms_since_unix_epoch",
            skip_serializing_if = "Option::is_none"
        )]
        pub expiration_ts: Option<SystemTime>,

        /// This is an optional parameter, it enables/disables sending renewal emails to the user.
        /// Defaults to true.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub enable_renewal_emails: Option<bool>,
    }

    response: {
        /// The new expiration date for this account, as a timestamp in milliseconds since epoch
        #[serde(with = "ruma_serde::time::ms_since_unix_epoch")]
        pub expiration_ts: SystemTime,
    }
}

impl<'a> Request<'a> {
    /// Creates an empty `Request`.
    pub fn new(user_id: &'a UserId) -> Self {
        Self {
            user_id,
            expiration_ts: None,
            enable_renewal_emails: None,
        }
    }
}

impl Response {
    /// Creates a `Response` with the new expiration date for this account,
    /// as a timestamp in milliseconds since epoch.
    pub fn new(expiration_ts: SystemTime) -> Self {
        Self { expiration_ts }
    }
}

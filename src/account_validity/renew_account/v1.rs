//! [GET /_synapse/admin/v1/account_validity/validity](https://github.com/matrix-org/synapse/blob/master/docs/admin_api/account_validity.rst)

use ruma::{api::ruma_api, MilliSecondsSinceUnixEpoch, UserId};

ruma_api! {
    metadata: {
        description: "Extend the account validity.",
        method: POST,
        name: "renew_account_v1",
        unstable_path: "/_synapse/admin/v1/account_validity/validity",
        rate_limited: false,
        authentication: AccessToken,
    }

    request: {
        /// user ID for the account to renew
        pub user_id: &'a UserId,

        /// This is an optional parameter, it overrides the expiration date,
        /// which otherwise defaults to now + validity period configured at the server
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub expiration_ts: Option<MilliSecondsSinceUnixEpoch>,

        /// This is an optional parameter, it enables/disables sending renewal emails to the user.
        /// Defaults to true.
        #[serde(default = "ruma::serde::default_true", skip_serializing_if = "ruma::serde::is_true")]
        pub enable_renewal_emails: bool,
    }

    response: {
        /// The new expiration date for this account, as a timestamp in milliseconds since epoch
        pub expiration_ts: MilliSecondsSinceUnixEpoch,
    }
}

impl<'a> Request<'a> {
    /// Creates an `Request` with the given user ID.
    pub fn new(user_id: &'a UserId) -> Self {
        Self { user_id, expiration_ts: None, enable_renewal_emails: true }
    }
}

impl Response {
    /// Creates a `Response` with the new expiration date for this account,
    pub fn new(expiration_ts: MilliSecondsSinceUnixEpoch) -> Self {
        Self { expiration_ts }
    }
}

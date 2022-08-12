//! [POST /_synapse/admin/v1/register](https://matrix-org.github.io/synapse/latest/admin_api/register_api.html#shared-secret-registration)

use ruma::{api::ruma_api, OwnedDeviceId, OwnedServerName, OwnedUserId};
use serde::{Deserialize, Serialize};

ruma_api! {
    metadata: {
        description: "Shared-secret registration",
        method: POST,
        name: "shared_secret_register_v1",
        unstable_path: "/_synapse/admin/v1/register",
        rate_limited: false,
        authentication: None,
    }

    #[derive(Default)]
    request: {
        /// The nonce retrieved from the nonce endpoint.
        pub nonce: &'a str,

        /// Localpart for the account.
        pub username: &'a str,

        /// Display name for the account.
        pub displayname: &'a str,

        /// Password for the account.
        pub password: &'a str,

        /// Whether the account should be an admin.
        pub admin: bool,

        /// The MAC is the hex digest output of the HMAC-SHA1 algorithm, with
        /// the key being the shared secret and the content being the nonce,
        /// user, password, either the string "admin" or "notadmin", and
        /// optionally the user_type each separated by NULs.
        pub mac: &'a str,
    }

    response: {
        /// Access token.
        pub access_token: String,

        /// Registered user id.
        pub user_id: OwnedUserId,

        /// Homeserver name.
        pub home_server: OwnedServerName,

        /// Device id.
        pub device_id: OwnedDeviceId,
    }
}

impl<'a> Request<'a> {
    /// Creates a `Request` with the given data.
    pub fn new(
        nonce: &'a str,
        username: &'a str,
        displayname: &'a str,
        password: &'a str,
        admin: bool,
        mac: &'a str,
    ) -> Self {
        Self { nonce, username, displayname, password, admin, mac }
    }
}

impl Response {
    /// Creates a `Response` with the given data.
    pub fn new(
        access_token: String,
        user_id: OwnedUserId,
        home_server: OwnedServerName,
        device_id: OwnedDeviceId,
    ) -> Self {
        Self { access_token, user_id, home_server, device_id }
    }
}

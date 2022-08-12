//! [POST /_synapse/admin/v1/register](https://matrix-org.github.io/synapse/latest/admin_api/register_api.html#shared-secret-registration)

#[cfg(feature = "shared-secret-registration-mac")]
use hmac::{digest::InvalidLength, Hmac, Mac};
use ruma::{api::ruma_api, OwnedDeviceId, OwnedServerName, OwnedUserId};
use serde::{Deserialize, Serialize};
#[cfg(feature = "shared-secret-registration-mac")]
use sha1::Sha1;

#[cfg(feature = "shared-secret-registration-mac")]
type HmacSha1 = Hmac<Sha1>;

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
        /// Details about the registration.
        #[ruma_api(body)]
        pub details: RegisterDetails,
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
    /// Creates a `Response` with the given details.
    pub fn new(details: RegisterDetails) -> Self {
        Self { details }
    }
}

/// Details about the registration.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RegisterDetails {
    /// Access token.
    pub access_token: String,

    /// Registered user id.
    pub user_id: OwnedUserId,

    /// Homeserver name.
    pub home_server: OwnedServerName,

    /// Device ID.
    pub device_id: OwnedDeviceId,
}

/// Calculate the MAC based on the given inputs.
///
/// See <https://matrix-org.github.io/synapse/latest/admin_api/register_api.html#shared-secret-registration> for details.
#[cfg(feature = "shared-secret-registration-mac")]
pub fn hmac(
    registration_shared_secret: &str,
    nonce: &str,
    username: &str,
    password: &str,
    admin: bool,
) -> Result<String, InvalidLength> {
    let mut mac = HmacSha1::new_from_slice(registration_shared_secret.as_bytes())?;
    mac.update(nonce.as_bytes());
    mac.update(b"\x00");
    mac.update(username.as_bytes());
    mac.update(b"\x00");
    mac.update(password.as_bytes());
    mac.update(b"\x00");
    mac.update({
        if admin {
            b"admin"
        } else {
            b"notadmin"
        }
    });
    let mac = mac.finalize();
    let mac = hex::encode(mac.into_bytes());

    Ok(mac)
}

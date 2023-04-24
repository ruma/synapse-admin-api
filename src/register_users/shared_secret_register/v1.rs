//! [POST /_synapse/admin/v1/register](https://matrix-org.github.io/synapse/latest/admin_api/register_api.html#shared-secret-registration)

#[cfg(feature = "shared-secret-registration-mac")]
use hmac::{digest::InvalidLength, Hmac, Mac};
use ruma::{
    api::{metadata, request, response, Metadata},
    OwnedDeviceId, OwnedServerName, OwnedUserId,
};
#[cfg(feature = "shared-secret-registration-mac")]
use sha1::Sha1;

#[cfg(feature = "shared-secret-registration-mac")]
type HmacSha1 = Hmac<Sha1>;

const METADATA: Metadata = metadata! {
    method: POST,
    rate_limited: false,
    authentication: None,
    history: {
        unstable => "/_synapse/admin/v1/register",
    }
};

#[request]
#[derive(Default)]
pub struct Request {
    /// The nonce retrieved from the nonce endpoint.
    pub nonce: String,

    /// Localpart for the account.
    pub username: String,

    /// Display name for the account.
    pub displayname: String,

    /// Password for the account.
    pub password: String,

    /// Whether the account should be an admin.
    pub admin: bool,

    /// The MAC is the hex digest output of the HMAC-SHA1 algorithm, with
    /// the key being the shared secret and the content being the nonce,
    /// user, password, either the string "admin" or "notadmin", and
    /// optionally the user_type each separated by NULs.
    pub mac: String,
}

#[response]
pub struct Response {
    /// Access token.
    pub access_token: String,

    /// Registered user id.
    pub user_id: OwnedUserId,

    /// Homeserver name.
    pub home_server: OwnedServerName,

    /// Device ID.
    pub device_id: OwnedDeviceId,
}

impl Request {
    /// Creates a `Request` with the given data.
    pub fn new(
        nonce: String,
        username: String,
        displayname: String,
        password: String,
        admin: bool,
        mac: String,
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

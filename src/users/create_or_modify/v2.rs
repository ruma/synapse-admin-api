//! [PUT /_synapse/admin/v2/users/:user_id](https://github.com/matrix-org/synapse/blob/master/docs/admin_api/user_admin_api.rst#create-or-modify-account)

use ruma::{
    api::{metadata, request, response, Metadata},
    thirdparty::ThirdPartyIdentifier,
    OwnedUserId,
};

pub use crate::users::{ExternalId, UserDetails};

const METADATA: Metadata = metadata! {
    method: PUT,
    rate_limited: false,
    authentication: AccessToken,
    history: {
        unstable => "/_synapse/admin/v2/users/:user_id",
    }
};

#[request]
pub struct Request {
    /// User ID for the account to renew
    #[ruma_api(path)]
    pub user_id: OwnedUserId,

    /// This is an optional parameter. Add this parameter to create an account or set this
    /// password as new one for an existing account.
    pub password: Option<String>,

    // NOTE: Server explodes if attributes are not omitted but specified as null, like the default
    // Serde case.
    /// Defaults to user_id, or the current value if user already exists
    /// Some("") is treated as setting it to null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub displayname: Option<String>,

    /// Defaults to empty, or the current value if user already exists
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threepids: Option<Vec<ThirdPartyIdentifier>>,

    /// Defaults to empty, or the current value if user already exists
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_ids: Option<Vec<ExternalId>>,

    /// The user's avatar URL, if set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,

    /// Should the user be a server admin
    /// defaults to false, or the current value if user already exists
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin: Option<bool>,

    /// Should the user be deactivated
    /// defaults to false, or the current value if user already exists
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deactivated: Option<bool>,
}

#[response]
pub struct Response {
    /// Details about the user.
    #[ruma_api(body)]
    pub details: UserDetails,
}

// todo following to does are from synadminctl
// TODO: returns 200 if account-exist-and-was-updated,
// but 201 CREATED if a new account was created.
// However, ruma does throw away this information.

// TODO: what do the EndpointErrors?
// -> can I add custom code, which converts http::Response into ruma embedded error type
// The error is necessary at least at all endpoints which need auth, because a invalid login
// response such an error
// TODO: Should this be the real error like at ruma client api error, is Void-Default enough?
// TODO: ruma api serialisis Ok if status code < 400, alse error. That should be diskussed.
// The redirect 300 area is Ok too.

impl Request {
    /// Creates a Request with the user ID and the optional password.
    pub fn new(user_id: OwnedUserId, password: Option<String>) -> Self {
        Self {
            user_id,
            password,
            displayname: None,
            threepids: None,
            external_ids: None,
            avatar_url: None,
            admin: None,
            deactivated: None,
        }
    }
}

impl Response {
    /// Creates a new `Response` with the user details.
    pub fn new(details: UserDetails) -> Self {
        Self { details }
    }
}

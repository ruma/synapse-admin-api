//! [GET /_synapse/admin/v2/users/:user_id](https://github.com/matrix-org/synapse/blob/master/docs/admin_api/user_admin_api.rst#create-or-modify-account)

pub use crate::users::UserDetails;
use ruma::{
    api::{
        client::{r0::contact::get_contacts::ThirdPartyIdentifier, Error},
        ruma_api,
    },
    UserId,
};

ruma_api! {
    metadata: {
        description: "create or modify account endpoint",
        method: PUT,
        name: "create_modify_account",
        path: "/_synapse/admin/v2/users/:user_id",
        rate_limited: false,
        authentication: AccessToken,
    }

    request: {
        /// user ID for the account to renew
        #[ruma_api(path)]
        pub user_id: &'a UserId,

        /// This is an optional parameter. Add this parameter to create an account or set this
        /// password as new one for an existing account.
        pub password: Option<&'a str>,

        // NOTE: Server explodes if attributes are not omitted but specified as null, like the default
        // Serde case.

        /// defaults to user_id, or the current value if user already exists
        /// Some("") is treated as setting it to null.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub displayname: Option<String>,

        /// defaults to empty, or the current value if user already exists
        #[serde(skip_serializing_if = "Option::is_none")]
        pub threepids: Option<Vec<ThirdPartyIdentifier>>,

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

    response: {
        /// Details about the user.
        #[ruma_api(body)]
        pub details: UserDetails,
    }

    // temporary workaround until
    // https://github.com/matrix-org/matrix-rust-sdk/issues/125
    // is solved
    error: Error

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
}

impl<'a> Request<'a> {
    /// Creates a Request with the user ID and the optional password.
    pub fn new(user_id: &'a UserId, password: Option<&'a str>) -> Self {
        Self {
            user_id,
            password,
            displayname: None,
            threepids: None,
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

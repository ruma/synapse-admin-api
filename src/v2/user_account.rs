use std::time::SystemTime;

use serde::{Serialize, Deserialize};

use ruma::{
    api::{
        client::{
            self,
            r0::contact::get_contacts::ThirdPartyIdentifier,
        },
        ruma_api,
    },
    identifiers::UserId,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserInformation {
    /// The user's name.
    pub name: String,

    /// The password hash of th account
    pub password_hash: String,

    /// Is the account a guest
    pub is_guest: u64,

    /// Is the user a server admin
    pub admin: u64,

    // todo: doc but I do not know what this is
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent_version: Option<String>,

    // todo: doc but I do not know what this is
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent_server_notice_sent: Option<bool>,

    // todo: doc but I do not know what this is
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appservice_id: Option<String>,

    /// creation date for the account
    // todo: how to get rid of this option?
    #[serde(with = "ruma::serde::time::opt_s_since_unix_epoch")]
    pub creation_ts: Option<SystemTime>,

    // todo: doc but I do not know what this is
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_type: Option<String>,

    /// Is the account deactivated
    pub deactivated: u64,

    /// The user's display name, if set.
    pub displayname: String,

    /// The user's avatar URL, if set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,

    /// A list of third party identifiers the homeserver has associated with the user's
    /// account.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub threepids: Vec<ThirdPartyIdentifier>,
}



pub mod query_user {
    //! [GET /_synapse/admin/v2/users/:user_id](https://github.com/matrix-org/synapse/blob/master/docs/admin_api/user_admin_api.rst#query-user-account)
    use super::*;

    ruma_api! {
        metadata: {
            description: "Get information about a specific user account.",
            method: GET,
            name: "query_user_account",
            path: "/_synapse/admin/v2/users/:user_id",
            rate_limited: false,
            authentication: AccessToken,
        }

        request: {
            /// user ID
            #[ruma_api(path)]
            pub user_id: &'a UserId,
        }

        response: {
            /// Information about the user.
            #[ruma_api(body)]
            pub info: UserInformation,
        }

        // temporary workaround until
        // https://github.com/matrix-org/matrix-rust-sdk/issues/125
        // is solved
        error: client::Error

    }

    impl<'a> Request<'a> {
        /// Creates an `Request` with the given user ID.
        pub fn new(user_id: &'a UserId) -> Self {
            Self { user_id }
        }
    }

    impl Response {
        /// Creates a new `Response` with all parameters defaulted.
        pub fn new(info: UserInformation) -> Self {
            Self { info }
        }
    }
}


pub mod create_modify_user {
    //! [GET /_synapse/admin/v2/users/:user_id](https://github.com/matrix-org/synapse/blob/master/docs/admin_api/user_admin_api.rst#create-or-modify-account)
    use super::*;

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
            pub threepids: Option<Vec<super::ThirdPartyIdentifier>>,

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
            /// Information about the user.
            #[ruma_api(body)]
            pub info: UserInformation,
        }

        // temporary workaround until
        // https://github.com/matrix-org/matrix-rust-sdk/issues/125
        // is solved
        error: client::Error

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
        pub fn new(user_id: &'a UserId, password: Option<&'a str>, ) -> Self {
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
        /// Creates a new `Response` with all parameters defaulted.
        pub fn new(info: UserInformation) -> Self {
            Self { info }
        }
    }
}


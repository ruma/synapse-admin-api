//! [GET /_synapse/admin/v2/users/:user_id](https://github.com/matrix-org/synapse/blob/master/docs/admin_api/user_admin_api.rst#list-accountshttps://github.com/matrix-org/synapse/blob/master/docs/admin_api/user_admin_api.rst#query-user-account)

use crate::serde::boolean_as_uint;
use ruma::{api::ruma_api, UInt};
use serde::{Deserialize, Serialize};

ruma_api! {
    metadata: {
        description: "list users endpoint",
        method: GET,
        name: "list_users",
        path: "/_synapse/admin/v2/users",
        rate_limited: false,
        authentication: AccessToken,
    }

    #[derive(Default)]
    request: {
        /// Offset in the returned list.
        ///
        /// Defaults to 0.
        #[serde(default, skip_serializing_if = "ruma::serde::is_default")]
        #[ruma_api(query)]
        pub from: UInt,

        /// Maximum amount of users to return. Defaults to 100.
        #[serde(skip_serializing_if = "Option::is_none")]
        #[ruma_api(query)]
        pub limit: Option<UInt>,

        /// user_id is optional and filters to only return users with user IDs that contain this value.
        ///
        /// This parameter is ignored when using the name parameter.
        #[serde(skip_serializing_if = "Option::is_none")]
        #[ruma_api(query)]
        pub user_id: Option<&'a UserId>,

        /// name is optional and filters to only return users with user ID localparts or displaynames that contain this value.
        #[serde(skip_serializing_if = "Option::is_none")]
        #[ruma_api(query)]
        pub name: Option<&'a str>,

        /// The parameter guests is optional and if false will exclude guest users.
        ///
        /// Defaults to true to include guest users.
        #[serde(default = "ruma::serde::default_true", skip_serializing_if = "ruma::serde::is_true")]
        #[ruma_api(query)]
        pub guests: bool,

        /// The parameter deactivated is optional and if true will include deactivated users.
        ///
        /// Defaults to false to exclude deactivated users.
        #[serde(default, skip_serializing_if = "ruma::serde::is_default")]
        #[ruma_api(query)]
        pub deactivated: bool,
    }

    response: {
        /// List of users containing `UserMinorDetails`.
        pub users: Vec<UserMinorDetails>,

        /// Token to receive the next `UserMinorDetails` batch.
        ///
        /// To paginate, check for next_token and if present, call the endpoint again with from set
        /// to the value of next_token. This will return a new page. If the endpoint does not return
        /// a next_token then there are no more users to paginate through.
        pub next_token: Option<String>,

        /// Total amount of users.
        pub total: UInt,
    }
}

impl Request {
    /// Creates an empty `Request`.
    pub fn new() -> Self {
        Default::default()
    }
}

impl Response {
    /// Creates a `Response` with the given `UserMinorDetails` and the total amount of users.
    pub fn new(users: Vec<UserMinorDetails>, total: UInt) -> Self {
        Self { users, next_token: None, total }
    }
}

/// A minor set of user details.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserMinorDetails {
    /// The user's name.
    pub name: String,

    /// todo: doc but I do not know what this is
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_type: Option<String>,

    /// Is the account a guest
    #[serde(with = "boolean_as_uint")]
    pub is_guest: bool,

    /// Is the user a server admin
    #[serde(with = "boolean_as_uint")]
    pub admin: bool,

    /// Is the account deactivated
    #[serde(with = "boolean_as_uint")]
    pub deactivated: bool,

    /// The user's display name, if set.
    pub displayname: String,

    /// The user's avatar URL, if set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
}

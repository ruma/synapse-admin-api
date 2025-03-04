//! Endpoints in the `/_synapse/admin/v<x>/users/` scope.

pub mod create_or_modify;
pub mod get_details;
pub mod is_user_admin;
pub mod list_joined_rooms;
pub mod list_users;
pub mod reset_password;

use ruma::{thirdparty::ThirdPartyIdentifier, SecondsSinceUnixEpoch};
use serde::{Deserialize, Serialize};

/// User details
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(not(ruma_unstable_exhaustive_types), non_exhaustive)]
pub struct UserDetails {
    /// The user's name.
    pub name: String,

    /// The password hash of the account
    pub password_hash: Option<String>,

    /// Is the account a guest
    #[serde(deserialize_with = "crate::serde::bool_or_uint")]
    pub is_guest: bool,

    /// Is the user a server admin
    #[serde(deserialize_with = "crate::serde::bool_or_uint")]
    pub admin: bool,

    /// todo: doc but I do not know what this is
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent_version: Option<String>,

    /// todo: doc but I do not know what this is
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent_server_notice_sent: Option<bool>,

    /// todo: doc but I do not know what this is
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appservice_id: Option<String>,

    /// creation date for the account
    // todo: how to get rid of this option?
    pub creation_ts: Option<SecondsSinceUnixEpoch>,

    /// todo: doc but I do not know what this is
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_type: Option<String>,

    /// Is the account deactivated
    #[serde(deserialize_with = "crate::serde::bool_or_uint")]
    pub deactivated: bool,

    /// The user's display name, if set.
    pub displayname: String,

    /// The user's avatar URL, if set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,

    /// A list of third party identifiers the homeserver has associated with the user.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub threepids: Vec<ThirdPartyIdentifier>,

    /// A list of external auth identifiers the homeserver has associated with the user.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub external_ids: Vec<ExternalId>,

    /// Is the account locked
    #[serde(default, deserialize_with = "crate::serde::bool_or_uint")]
    pub locked: bool,
}

impl UserDetails {
    /// Construct a `UserDetails` with the given user name and all the other fields set to their
    /// default value.
    pub fn new(name: String) -> Self {
        Self {
            name,
            password_hash: None,
            is_guest: false,
            admin: false,
            consent_version: None,
            consent_server_notice_sent: None,
            appservice_id: None,
            creation_ts: None,
            user_type: None,
            deactivated: false,
            displayname: String::new(),
            avatar_url: None,
            threepids: Vec::new(),
            external_ids: Vec::new(),
            locked: false,
        }
    }
}

/// An external ID associated with a user
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(not(ruma_unstable_exhaustive_types), non_exhaustive)]
pub struct ExternalId {
    /// The authentication provider to which the user is associated.
    pub auth_provider: String,

    /// The ID known to the auth provider associated with this user.
    pub external_id: String,
}

impl ExternalId {
    /// Construct an `ExternalId` with the given authentication provider and ID.
    pub fn new(auth_provider: String, external_id: String) -> Self {
        Self { auth_provider, external_id }
    }
}

//! Endpoints in the `/_synapse/admin/v<x>/users/` scope.

pub mod create_or_modify;
pub mod get_details;
pub mod is_user_admin;
pub mod list_joined_rooms;
pub mod list_users;
pub mod reset_password;

use crate::serde::boolean_as_uint;
use ruma::{api::client::r0::contact::get_contacts::ThirdPartyIdentifier, SecondsSinceUnixEpoch};
use serde::{Deserialize, Serialize};

/// User details
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserDetails {
    /// The user's name.
    pub name: String,

    /// The password hash of th account
    pub password_hash: String,

    /// Is the account a guest
    #[serde(with = "boolean_as_uint")]
    pub is_guest: bool,

    /// Is the user a server admin
    #[serde(with = "boolean_as_uint")]
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
    #[serde(with = "boolean_as_uint")]
    pub deactivated: bool,

    /// The user's display name, if set.
    pub displayname: String,

    /// The user's avatar URL, if set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,

    /// A list of third party identifiers the homeserver has associated with the user.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub threepids: Vec<ThirdPartyIdentifier>,
}

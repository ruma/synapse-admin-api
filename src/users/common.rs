//! Common parts for the user endpoints.

use ruma::api::client::r0::contact::get_contacts::ThirdPartyIdentifier;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// User details
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserDetails {
    /// The user's name.
    pub name: String,

    /// The password hash of th account
    pub password_hash: String,

    /// Is the account a guest
    pub is_guest: u64,

    /// Is the user a server admin
    pub admin: u64,

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
    #[serde(with = "ruma::serde::time::opt_s_since_unix_epoch")]
    pub creation_ts: Option<SystemTime>,

    /// todo: doc but I do not know what this is
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_type: Option<String>,

    /// Is the account deactivated
    pub deactivated: u64,

    /// The user's display name, if set.
    pub displayname: String,

    /// The user's avatar URL, if set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,

    /// A list of third party identifiers the homeserver has associated with the user.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub threepids: Vec<ThirdPartyIdentifier>,
}

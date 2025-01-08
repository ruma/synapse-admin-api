//! Serializable types for the requests and responses for each endpoint in the
//! [synapse admin API][api].
//!
//! [api]: https://github.com/matrix-org/synapse/tree/master/docs/admin_api

// FIXME: don't allow dead code, warn on missing docs
#![allow(dead_code)]
#![warn(missing_debug_implementations)]

use std::fmt;

pub mod account_validity;
pub mod background_updates;
pub mod experimental_features;
pub mod register_users;
pub mod room_membership;
pub mod rooms;
pub mod users;
pub mod version;

mod serde;

// Wrapper around `Box<str>` that cannot be used in a meaningful way outside of
// this crate. Used for string enums because their `_Custom` variant can't be
// truly private (only `#[doc(hidden)]`).
#[doc(hidden)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PrivOwnedStr(Box<str>);

impl fmt::Debug for PrivOwnedStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

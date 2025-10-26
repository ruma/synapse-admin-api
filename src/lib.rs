//! Serializable types for the requests and responses for each endpoint in the
//! [synapse admin API][api].
//!
//! # Compile-time `cfg` settings
//!
//! These settings are accepted at compile time to configure the generated code. They can be set as
//! `--cfg={key}={value}` using `RUSTFLAGS` or `.cargo/config.toml` (under `[build]` -> `rustflags =
//! ["..."]`).
//!
//! * `ruma_identifiers_storage` -- Choose the inner representation of `Owned*` wrapper types for
//!   identifiers. By default they use [`Box`], setting the value to `Arc` makes them use
//!   [`Arc`](std::sync::Arc).
//! * `ruma_unstable_exhaustive_types` -- Most types in synapse-admin-api are marked as
//!   non-exhaustive to avoid breaking changes when new fields are added in the API. This setting
//!   compiles all types as exhaustive. By enabling this feature you opt out of all semver
//!   guarantees synapse-admin-api otherwise provides.
//!
//! [api]: https://github.com/element-hq/synapse/tree/master/docs/admin_api

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

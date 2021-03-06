//! Serializable types for the requests and responses for each endpoint in the
//! [synapse admin API][api].
//!
//! [api]: https://github.com/matrix-org/synapse/tree/master/docs/admin_api
#![warn(missing_debug_implementations, missing_docs)]

pub mod account_validity;
pub mod rooms;
pub mod users;
pub mod version;

mod serde;

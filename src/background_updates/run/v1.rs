//! [POST /_synapse/admin/v1/background_updates/start_job](https://github.com/element-hq/synapse/blob/master/docs/usage/administration/admin_api/background_updates.md#run)

use ruma::{
    api::{request, response, Metadata},
    metadata,
    serde::StringEnum,
};

const METADATA: Metadata = metadata! {
    method: POST,
    rate_limited: false,
    authentication: AccessToken,
    history: {
        unstable => "/_synapse/admin/v1/background_updates/start_job",
    }
};

#[request]
pub struct Request {
    /// A string which job to run
    pub job_name: JobName,
}

#[response]
#[derive(Default)]
pub struct Response {}

impl Request {
    /// Creates a `Request` with the given `job_name` value.
    pub fn new(job_name: JobName) -> Self {
        Self { job_name }
    }
}

impl Response {
    /// Creates an empty `Response`.
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Clone, PartialEq, StringEnum)]
#[ruma_enum(rename_all = "snake_case")]
pub enum JobName {
    /// Recalculate the stats for all rooms.
    PopulateStatsProcessRooms,
    /// Recalculate the user directory if it is stale or out of sync
    RegenerateDirectory,

    #[doc(hidden)]
    _Custom(crate::PrivOwnedStr),
}

#[test]
fn test_run_background_updates() {
    let job_name = JobName::PopulateStatsProcessRooms;
    // Check create request
    let request = Request::new(job_name.clone());
    assert_eq!(request.job_name, job_name);
}

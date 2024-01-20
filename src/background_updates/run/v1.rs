//! [POST /_synapse/admin/v1/background_updates/start_job](https://github.com/element-hq/synapse/blob/master/docs/usage/administration/admin_api/background_updates.md#run)

use ruma::api::{Metadata, request, response};
use ruma::metadata;

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
    ///
    /// Valid values are:
    /// populate_stats_process_rooms
    /// regenerate_directory
    pub job_name: String
}

#[response]
pub struct Response {}

impl Request {
    /// Creates a `Request` with the given `job_name` value.
    pub fn new(job_name: String) -> Self { Self { job_name } }
}

impl Response {
    /// Creates an empty `Response`.
    pub fn new() -> Self {
        Self {}
    }
}

#[test]
fn test_run_background_updates() {
    let job_name = "test-job-name".to_string();
    // Check create request
    let request = Request::new(job_name.clone());
    assert_eq!(request.job_name, job_name);
}
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExternalTaskFailureDto {
    /// The id of the worker that reports the failure. Must match the id of the worker who has most recently locked the task.
    #[serde(rename = "workerId", skip_serializing_if = "Option::is_none")]
    pub worker_id: Option<String>,
    /// An message indicating the reason of the failure.
    #[serde(rename = "errorMessage", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,

    //error_details removed in version 7.5
    // /// A detailed error description.
    // #[serde(rename = "errorDetails", skip_serializing_if = "Option::is_none")]
    // pub error_details: Option<String>,
    /// A number of how often the task should be retried. Must be >= 0. If this is 0, an incident is created and the task cannot be fetched anymore unless the retries are increased again. The incident's message is set to the `errorMessage` parameter.
    #[serde(rename = "retries", skip_serializing_if = "Option::is_none")]
    pub retries: Option<i32>,
    /// A timeout in milliseconds before the external task becomes available again for fetching. Must be >= 0.
    #[serde(rename = "retryTimeout", skip_serializing_if = "Option::is_none")]
    pub retry_timeout: Option<i64>,
}

impl ExternalTaskFailureDto {
    pub fn new() -> ExternalTaskFailureDto {
        ExternalTaskFailureDto {
            worker_id: None,
            error_message: None,
            retries: None,
            retry_timeout: None,
        }
    }
}

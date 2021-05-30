use reqwest;
use serde_json;
use thiserror::Error;

use crate::ProcessVariablesMap;

#[derive(Debug, Error)]
pub enum CamundaClientError {
    #[error("reqwest::Error:  {}", .0)]
    Reqwest(#[from] reqwest::Error),
    #[error("serde_json::Error:  {}", .0)]
    Serde(#[from] serde_json::Error),
    #[error("std::io::Error:  {}", .0)]
    Io(#[from] std::io::Error),
}

#[derive(Debug)]
pub struct CamundaProcessError {
    pub code: String,
    pub error: String,
    pub variables: Option<ProcessVariablesMap>,
}

impl Error for CamundaProcessError {}

impl fmt::Display for CamundaProcessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "BPMN Error Occured! Error Code: {:?} Error Message: {:?}",
            self.code, self.error
        )
    }
}

#[derive(Debug)]
pub struct CamundaProcessFailure {
    pub error: String,
    pub retries: i32,
    pub retry_timeout: i64,
}

impl Error for CamundaProcessFailure {}

impl fmt::Display for CamundaProcessFailure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Job Failed! Error: {:?}. The job will be retried {:?} more times with {:?} milliseconds intervals", self.error, self.retries, self.retry_timeout
        )
    }
}

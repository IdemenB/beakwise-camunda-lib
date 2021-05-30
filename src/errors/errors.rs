use std::{error::Error, fmt};

use reqwest;
use serde_json;

use crate::ProcessVariablesMap;

#[derive(Debug)]
pub enum CamundaClientError {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
}

impl From<reqwest::Error> for CamundaClientError {
    fn from(e: reqwest::Error) -> Self {
        CamundaClientError::Reqwest(e)
    }
}

impl From<serde_json::Error> for CamundaClientError {
    fn from(e: serde_json::Error) -> Self {
        CamundaClientError::Serde(e)
    }
}

impl From<std::io::Error> for CamundaClientError {
    fn from(e: std::io::Error) -> Self {
        CamundaClientError::Io(e)
    }
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

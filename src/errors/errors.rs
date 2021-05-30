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

#[derive(Debug, Error)]
#[error("CamundaProcessError: Code: {:?} Error: {:?} Variables: {:?}", self.code, self.error, self.variables)]
pub struct CamundaProcessError {
    pub code: String,
    pub error: String,
    pub variables: Option<ProcessVariablesMap>,
}

#[derive(Debug, Error)]
#[error("CamundaProcessFailure: Error: {:?} Retries: {:?} Retry Timeout: {:?} ms", self.error, self.retries, self.retry_timeout)]
pub struct CamundaProcessFailure {
    pub error: String,
    pub retries: i32,
    pub retry_timeout: i64,
}

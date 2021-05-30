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

#[derive(Debug)]
pub struct CamundaProcessFailure {
    pub error: String,
    pub retries: i32,
    pub retry_timeout: i64,
}

impl Error for CamundaProcessFailure {}

// #[derive(Debug)]
// pub enum Error {
//     Reqwest(reqwest::Error),
//     Serde(serde_json::Error),
//     Io(std::io::Error),
// }

// impl From<reqwest::Error> for Error {
//     fn from(e: reqwest::Error) -> Self {
//         Error::Reqwest(e)
//     }
// }

// impl From<serde_json::Error> for Error {
//     fn from(e: serde_json::Error) -> Self {
//         Error::Serde(e)
//     }
// }

// impl From<std::io::Error> for Error {
//     fn from(e: std::io::Error) -> Self {
//         Error::Io(e)
//     }
// }

use crate::models::VariableValueDto;
use reqwest::{header::InvalidHeaderValue, Error as ReqwestError};
use serde_json;
use std::{collections::HashMap, fmt};
use thiserror::Error;
use tokio::task::JoinError;

pub type VariablesMap = HashMap<String, VariableValueDto>;

#[derive(Debug)]
pub struct BpmErrorEventError {
    pub code: String,
    pub error: String,
    pub variables: Option<VariablesMap>,
}

impl fmt::Display for BpmErrorEventError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "A business process error occurred while running the process"
        )
    }
}

#[derive(Debug, Error)]
pub enum WorkflowError {
    #[error("Error serializing to JSON. Error message: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("Error while performing IO operation. Error message: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Error while making the HTTP connection. Error message: {0}")]
    ReqwestError(#[from] ReqwestError),
    #[error("Error with Headers of the HTTP request. Error message: {0} ")]
    InvalidHeaderValueError(#[from] InvalidHeaderValue),

    // #[error("Error while unwrapping an optional value. Value not found. Error message: {0} ")]
    // NoneError(#[from] NoneError),
    #[error("Error while awaiting the spawned Thread. Error message: {0} ")]
    JoinError(#[from] JoinError),
    #[error("Error occured while reading the environment variables. Error message: {0} ")]
    DotEnvError(#[from] dotenv::Error),
    #[error(
        "A business-related error occured. Error Code: {} Error Message: {}",
        .0.code,
        .0.error
    )]
    BpmError(BpmErrorEventError),
}

impl From<BpmErrorEventError> for WorkflowError {
    fn from(e: BpmErrorEventError) -> Self {
        WorkflowError::BpmError(e)
    }
}

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

use crate::worker::VariablesMap;
use anyhow::Error as AnyhowError;
use csv;
use r2d2_oracle::oracle::Error as OracleError;
use reqwest::{header::InvalidHeaderValue, Error as ReqwestError};
use serde_json;
use sqlx::Error as SqlxError;
use std::{
    error::Error,
    fmt,
    num::{ParseIntError, TryFromIntError},
};
use thiserror::Error;
use tokio::task::JoinError;
use yaserde_derive::{YaDeserialize, YaSerialize};
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

#[derive(Debug)]
pub struct CommunicationGatewayError {
    pub error_code: String,
    pub error_message: String,
}

impl fmt::Display for CommunicationGatewayError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "An error occured on communication gateway. Error code: {:?}, Error Message: {:?}",
            self.error_code, self.error_message
        )
    }
}

impl Error for CommunicationGatewayError {}

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
    rename = "Fault",
    namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
    prefix = "soapenv"
)]
pub struct SoapFault {
    #[yaserde(rename = "faultcode", default)]
    pub fault_code: String,
    #[yaserde(rename = "faultstring", default)]
    pub fault_string: String,
}

impl fmt::Display for SoapFault {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "An error occured with the SOAP request. Error code: {:?}, Error Message: {:?}",
            self.fault_code, self.fault_string
        )
    }
}

impl Error for SoapFault {}

impl From<ReqwestError> for SoapFault {
    fn from(e: ReqwestError) -> Self {
        SoapFault {
            fault_code: e.status().unwrap_or_default().to_string(),
            fault_string: format!("{}", e),
        }
    }
}

// TODO Currently all errors defined in ExternalEror are handled via default settings in worker.rs .__thread_local_inner!(
//     This could be improved by first converting all error types into BpmFailure type (using From trait) and then changing the
//     main Error type in HandlerResult as ProcessError where there are only two types, BpmFailure or BpmErrorEventError are avaiable.

// #[derive(Debug)]
// pub struct BpmFailure {
//     pub error: String,
//     pub error_details: Option<String>,
//     pub retries: Option<u8>,
//     pub retry_timeout: Option<i32>,
//     pub variables: Option<Vec<VariablesMap>>,
// }

// impl fmt::Display for BpmFailure {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "A technical failure occurred while running the process")
//     }
// }

#[derive(Debug, Error)]
pub enum WorkflowError {
    #[error("Error serializing to JSON. Error message: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("Error while performing IO operation. Error message: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Error while dealing with CSV file. Error message: {0}")]
    CsvError(#[from] csv::Error),
    #[error("Error while performing the DB operation through the SQLx crate. Error message: {0}")]
    SqlxError(#[from] SqlxError),
    #[error("Error while making the HTTP connection. Error message: {0}")]
    ReqwestError(#[from] ReqwestError),
    #[error("Error with Headers of the HTTP request. Error message: {0} ")]
    InvalidHeaderValueError(#[from] InvalidHeaderValue),
    #[error(
        "Error while performing the DB operation through the rust-oracle crate. Error message:{0}"
    )]
    OracleError(#[from] OracleError),

    #[error("Error while generating the SQL query. Error message: {0} ")]
    AnyhowError(#[from] AnyhowError),
    // #[error("Error while unwrapping an optional value. Value not found. Error message: {0} ")]
    // NoneError(#[from] NoneError),
    #[error("Error while awaiting the spawned Thread. Error message: {0} ")]
    JoinError(#[from] JoinError),
    #[error("Error occured while parsing from a string value. Error message: {0} ")]
    ParseIntError(#[from] ParseIntError),
    #[error("Error occured while downcasting from a bigger numeric value. Error message: {0} ")]
    TryFromIntError(#[from] TryFromIntError),
    #[error("Error occured while reading the environment variables. Error message: {0} ")]
    DotEnvError(#[from] dotenv::Error),
    #[error("Error occured on the communications gateway. Error Code: {:?}, Error Message: {:?}", .0.error_code, .0.error_message)]
    CommunicationGatewayError(#[from] CommunicationGatewayError),
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

// impl From<std::io::Error> for WorkflowError {
//     fn from(e: std::io::Error) -> Self {
//         WorkflowError::IoError(e)
//     }
// }

// impl From<csv::Error> for WorkflowError {
//     fn from(e: csv::Error) -> Self {
//         WorkflowError::CsvError(e)
//     }
// }

// impl From<camunda_client::apis::Error> for WorkflowError {
//     fn from(e: camunda_client::apis::Error) -> Self {
//         WorkflowError::CamundaClientError(e)
//     }
// }

// impl From<SqlxError> for WorkflowError {
//     fn from(e: SqlxError) -> Self {
//         WorkflowError::SqlxError(e)
//     }
// }

// impl From<ReqwestError> for WorkflowError {
//     fn from(e: ReqwestError) -> Self {
//         WorkflowError::ReqwestError(e)
//     }
// }

// impl From<InvalidHeaderValue> for WorkflowError {
//     fn from(e: InvalidHeaderValue) -> Self {
//         WorkflowError::InvalidHeaderValueError(e)
//     }
// }

// impl From<serde_json::Error> for ProcessError {
//     fn from(e: serde_json::Error) -> Self {
//         let mut process_error = ProcessError::new();
//         let external_error = WorkflowError::SerdeError(e);
//         process_error.error = Some(external_error);
//         process_error
//     }
// }

// impl From<std::io::Error> for ProcessError {
//     fn from(e: std::io::Error) -> Self {
//         let mut process_error = ProcessError::new();
//         let external_error = WorkflowError::IoError(e);
//         process_error.error = Some(external_error);
//         process_error
//     }
// }

// impl From<csv::Error> for ProcessError {
//     fn from(e: csv::Error) -> Self {
//         let mut process_error = ProcessError::new();
//         let external_error = WorkflowError::CsvError(e);
//         process_error.error = Some(external_error);
//         process_error
//     }
// }

// impl From<camunda_client::apis::Error> for ProcessError {
//     fn from(e: camunda_client::apis::Error) -> Self {
//         let mut process_error = ProcessError::new();
//         let external_error = WorkflowError::CamundaClientError(e);
//         process_error.error = Some(external_error);
//         process_error
//     }
// }

// impl From<SqlxError> for ProcessError {
//     fn from(e: SqlxError) -> Self {
//         let mut process_error = ProcessError::new();
//         let external_error = WorkflowError::SqlxError(e);
//         process_error.error = Some(external_error);
//         process_error
//     }
// }

// impl From<ReqwestError> for ProcessError {
//     fn from(e: ReqwestError) -> Self {
//         let mut process_error = ProcessError::new();
//         let external_error = WorkflowError::ReqwestError(e);
//         process_error.error = Some(external_error);
//         process_error
//     }
// }

// impl From<InvalidHeaderValue> for ProcessError {
//     fn from(e: InvalidHeaderValue) -> Self {
//         let mut process_error = ProcessError::new();
//         let external_error = WorkflowError::InvalidHeaderValueError(e);
//         process_error.error = Some(external_error);
//         process_error
//     }
// }

// impl From<std::option::NoneError> for WorkflowError {
//     fn from(e: std::option::NoneError) -> Self {
//         WorkflowError::NoneError(e)
//     }
// }

use crate::ProcessVariablesMap;
use reqwest;
use serde_json;
use thiserror::Error;

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

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

mod condition_api;
pub use self::condition_api::{ConditionApi, ConditionApiClient};
mod decision_evaluation_api;
pub use self::decision_evaluation_api::{DecisionEvaluationApi, DecisionEvaluationApiClient};
mod deployment_api;
pub use self::deployment_api::{DeploymentApi, DeploymentApiClient};
mod engine_api;
pub use self::engine_api::{EngineApi, EngineApiClient};
mod event_subscription_api;
pub use self::event_subscription_api::{EventSubscriptionApi, EventSubscriptionApiClient};
mod external_task_api;
pub use self::external_task_api::{ExternalTaskApi, ExternalTaskApiClient};
mod message_api;
pub use self::message_api::{MessageApi, MessageApiClient};
mod metrics_api;
pub use self::metrics_api::{MetricsApi, MetricsApiClient};
mod process_definition_api;
pub use self::process_definition_api::{ProcessDefinitionApi, ProcessDefinitionApiClient};
mod process_instance_api;
pub use self::process_instance_api::{ProcessInstanceApi, ProcessInstanceApiClient};
mod schema_log_api;
pub use self::schema_log_api::{SchemaLogApi, SchemaLogApiClient};
mod signal_api;
pub use self::signal_api::{SignalApi, SignalApiClient};
mod task_api;
pub use self::task_api::{TaskApi, TaskApiClient};
mod task_attachment_api;
pub use self::task_attachment_api::{TaskAttachmentApi, TaskAttachmentApiClient};
mod task_comment_api;
pub use self::task_comment_api::{TaskCommentApi, TaskCommentApiClient};
mod task_identity_link_api;
pub use self::task_identity_link_api::{TaskIdentityLinkApi, TaskIdentityLinkApiClient};
mod task_local_variable_api;
pub use self::task_local_variable_api::{TaskLocalVariableApi, TaskLocalVariableApiClient};
mod task_variable_api;
pub use self::task_variable_api::{TaskVariableApi, TaskVariableApiClient};
mod version_api;
pub use self::version_api::{VersionApi, VersionApiClient};

pub mod client;
pub mod configuration;

use super::configuration::Configuration;
pub use reqwest;
pub use serde;
pub use serde_derive;
pub use serde_json;
pub use url;

use std::rc::Rc;

pub struct APIClient {
    condition_api: Box<dyn crate::apis::ConditionApi>,
    deployment_api: Box<dyn crate::apis::DeploymentApi>,
    decision_evaluation_api: Box<dyn crate::apis::DecisionEvaluationApi>,
    engine_api: Box<dyn crate::apis::EngineApi>,
    event_subscription_api: Box<dyn crate::apis::EventSubscriptionApi>,
    external_task_api: Box<dyn crate::apis::ExternalTaskApi>,
    message_api: Box<dyn crate::apis::MessageApi>,
    metrics_api: Box<dyn crate::apis::MetricsApi>,
    process_definition_api: Box<dyn crate::apis::ProcessDefinitionApi>,
    process_instance_api: Box<dyn crate::apis::ProcessInstanceApi>,
    schema_log_api: Box<dyn crate::apis::SchemaLogApi>,
    signal_api: Box<dyn crate::apis::SignalApi>,
    task_api: Box<dyn crate::apis::TaskApi>,
    task_attachment_api: Box<dyn crate::apis::TaskAttachmentApi>,
    task_comment_api: Box<dyn crate::apis::TaskCommentApi>,
    task_identity_link_api: Box<dyn crate::apis::TaskIdentityLinkApi>,
    task_local_variable_api: Box<dyn crate::apis::TaskLocalVariableApi>,
    task_variable_api: Box<dyn crate::apis::TaskVariableApi>,
    version_api: Box<dyn crate::apis::VersionApi>,
}

impl APIClient {
    pub fn new(configuration: Configuration) -> APIClient {
        let rc = Rc::new(configuration);

        APIClient {
            condition_api: Box::new(crate::apis::ConditionApiClient::new(rc.clone())),
            decision_evaluation_api: Box::new(crate::apis::DecisionEvaluationApiClient::new(
                rc.clone(),
            )),
            deployment_api: Box::new(crate::apis::DeploymentApiClient::new(rc.clone())),
            engine_api: Box::new(crate::apis::EngineApiClient::new(rc.clone())),
            event_subscription_api: Box::new(crate::apis::EventSubscriptionApiClient::new(
                rc.clone(),
            )),
            external_task_api: Box::new(crate::apis::ExternalTaskApiClient::new(rc.clone())),
            message_api: Box::new(crate::apis::MessageApiClient::new(rc.clone())),
            metrics_api: Box::new(crate::apis::MetricsApiClient::new(rc.clone())),
            process_definition_api: Box::new(crate::apis::ProcessDefinitionApiClient::new(
                rc.clone(),
            )),
            process_instance_api: Box::new(crate::apis::ProcessInstanceApiClient::new(rc.clone())),
            schema_log_api: Box::new(crate::apis::SchemaLogApiClient::new(rc.clone())),
            signal_api: Box::new(crate::apis::SignalApiClient::new(rc.clone())),
            task_api: Box::new(crate::apis::TaskApiClient::new(rc.clone())),
            task_attachment_api: Box::new(crate::apis::TaskAttachmentApiClient::new(rc.clone())),
            task_comment_api: Box::new(crate::apis::TaskCommentApiClient::new(rc.clone())),
            task_identity_link_api: Box::new(crate::apis::TaskIdentityLinkApiClient::new(
                rc.clone(),
            )),
            task_local_variable_api: Box::new(crate::apis::TaskLocalVariableApiClient::new(
                rc.clone(),
            )),
            task_variable_api: Box::new(crate::apis::TaskVariableApiClient::new(rc.clone())),
            version_api: Box::new(crate::apis::VersionApiClient::new(rc.clone())),
        }
    }

    pub fn condition_api(&self) -> &dyn crate::apis::ConditionApi {
        self.condition_api.as_ref()
    }

    pub fn deployment_api(&self) -> &dyn crate::apis::DeploymentApi {
        self.deployment_api.as_ref()
    }

    pub fn decision_evaluation_api(&self) -> &dyn crate::apis::DecisionEvaluationApi {
        self.decision_evaluation_api.as_ref()
    }

    pub fn engine_api(&self) -> &dyn crate::apis::EngineApi {
        self.engine_api.as_ref()
    }

    pub fn event_subscription_api(&self) -> &dyn crate::apis::EventSubscriptionApi {
        self.event_subscription_api.as_ref()
    }

    pub fn external_task_api(&self) -> &dyn crate::apis::ExternalTaskApi {
        self.external_task_api.as_ref()
    }

    pub fn message_api(&self) -> &dyn crate::apis::MessageApi {
        self.message_api.as_ref()
    }

    pub fn metrics_api(&self) -> &dyn crate::apis::MetricsApi {
        self.metrics_api.as_ref()
    }

    pub fn process_definition_api(&self) -> &dyn crate::apis::ProcessDefinitionApi {
        self.process_definition_api.as_ref()
    }

    pub fn process_instance_api(&self) -> &dyn crate::apis::ProcessInstanceApi {
        self.process_instance_api.as_ref()
    }

    pub fn schema_log_api(&self) -> &dyn crate::apis::SchemaLogApi {
        self.schema_log_api.as_ref()
    }

    pub fn signal_api(&self) -> &dyn crate::apis::SignalApi {
        self.signal_api.as_ref()
    }

    pub fn task_api(&self) -> &dyn crate::apis::TaskApi {
        self.task_api.as_ref()
    }

    pub fn task_attachment_api(&self) -> &dyn crate::apis::TaskAttachmentApi {
        self.task_attachment_api.as_ref()
    }

    pub fn task_comment_api(&self) -> &dyn crate::apis::TaskCommentApi {
        self.task_comment_api.as_ref()
    }

    pub fn task_identity_link_api(&self) -> &dyn crate::apis::TaskIdentityLinkApi {
        self.task_identity_link_api.as_ref()
    }

    pub fn task_local_variable_api(&self) -> &dyn crate::apis::TaskLocalVariableApi {
        self.task_local_variable_api.as_ref()
    }

    pub fn task_variable_api(&self) -> &dyn crate::apis::TaskVariableApi {
        self.task_variable_api.as_ref()
    }

    pub fn version_api(&self) -> &dyn crate::apis::VersionApi {
        self.version_api.as_ref()
    }
}

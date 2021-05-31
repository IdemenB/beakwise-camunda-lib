use std::sync::Arc;

use super::configuration::Configuration;
pub use reqwest;
pub use serde;
pub use serde_derive;
pub use serde_json;
pub use url;

pub struct APIClient {
    // condition_api: Box<dyn crate::apis::ConditionApi + Send + Sync + 'static>,
    decision_evaluation_api: Box<dyn crate::apis::DecisionEvaluationApi + Send + Sync + 'static>,
    // deployment_api: Box<dyn crate::apis::DeploymentApi + Send + Sync + 'static>,
    // engine_api: Box<dyn crate::apis::EngineApi + Send + Sync + 'static>,
    // event_subscription_api: Box<dyn crate::apis::EventSubscriptionApi + Send + Sync + 'static>,
    external_task_api: Box<dyn crate::apis::ExternalTaskApi + Send + Sync + 'static>,
    // message_api: Box<dyn crate::apis::MessageApi + Send + Sync + 'static>,
    // metrics_api: Box<dyn crate::apis::MetricsApi + Send + Sync + 'static>,
    // process_definition_api: Box<dyn crate::apis::ProcessDefinitionApi + Send + Sync + 'static>,
    // process_instance_api: Box<dyn crate::apis::ProcessInstanceApi + Send + Sync + 'static>,
    // schema_log_api: Box<dyn crate::apis::SchemaLogApi + Send + Sync + 'static>,
    // signal_api: Box<dyn crate::apis::SignalApi + Send + Sync + 'static>,
    // task_api: Box<dyn crate::apis::TaskApi + Send + Sync + 'static>,
    // task_attachment_api: Box<dyn crate::apis::TaskAttachmentApi + Send + Sync + 'static>,
    // task_comment_api: Box<dyn crate::apis::TaskCommentApi + Send + Sync + 'static>,
    // task_identity_link_api: Box<dyn crate::apis::TaskIdentityLinkApi + Send + Sync + 'static>,
    // task_local_variable_api: Box<dyn crate::apis::TaskLocalVariableApi + Send + Sync + 'static>,
    // task_variable_api: Box<dyn crate::apis::TaskVariableApi + Send + Sync + 'static>,
    // version_api: Box<dyn crate::apis::VersionApi + Send + Sync + 'static>,
}

impl APIClient {
    pub fn new(configuration: Configuration) -> APIClient {
        let arc = Arc::new(configuration);

        APIClient {
            // condition_api: Box::new(crate::apis::ConditionApiClient::new(arc.clone())),
            // deployment_api: Box::new(crate::apis::DeploymentApiClient::new(arc.clone())),
            decision_evaluation_api: Box::new(crate::apis::DecisionEvaluationApiClient::new(
                arc.clone(),
            )),
            // engine_api: Box::new(crate::apis::EngineApiClient::new(arc.clone())),
            // event_subscription_api: Box::new(crate::apis::EventSubscriptionApiClient::new(
            //     arc.clone(),
            // )),
            external_task_api: Box::new(crate::apis::ExternalTaskApiClient::new(arc.clone())),
            // message_api: Box::new(crate::apis::MessageApiClient::new(arc.clone())),
            // metrics_api: Box::new(crate::apis::MetricsApiClient::new(arc.clone())),
            // process_definition_api: Box::new(crate::apis::ProcessDefinitionApiClient::new(
            //     arc.clone(),
            // )),
            // process_instance_api: Box::new(crate::apis::ProcessInstanceApiClient::new(arc.clone())),
            // schema_log_api: Box::new(crate::apis::SchemaLogApiClient::new(arc.clone())),
            // signal_api: Box::new(crate::apis::SignalApiClient::new(arc.clone())),
            // task_api: Box::new(crate::apis::TaskApiClient::new(arc.clone())),
            // task_attachment_api: Box::new(crate::apis::TaskAttachmentApiClient::new(arc.clone())),
            // task_comment_api: Box::new(crate::apis::TaskCommentApiClient::new(arc.clone())),
            // task_identity_link_api: Box::new(crate::apis::TaskIdentityLinkApiClient::new(
            //     arc.clone(),
            // )),
            // task_local_variable_api: Box::new(crate::apis::TaskLocalVariableApiClient::new(
            //     arc.clone(),
            // )),
            // task_variable_api: Box::new(crate::apis::TaskVariableApiClient::new(arc.clone())),
            // version_api: Box::new(crate::apis::VersionApiClient::new(arc.clone())),
        }
    }

    // pub fn condition_api(&self) -> &dyn crate::apis::ConditionApi {
    //     self.condition_api.as_ref()
    // }

    // pub fn deployment_api(&self) -> &dyn crate::apis::DeploymentApi {
    //     self.deployment_api.as_ref()
    // }

    pub fn decision_evaluation_api(&self) -> &dyn crate::apis::DecisionEvaluationApi {
        self.decision_evaluation_api.as_ref()
    }

    // pub fn engine_api(&self) -> &dyn crate::apis::EngineApi {
    //     self.engine_api.as_ref()
    // }

    // pub fn event_subscription_api(&self) -> &dyn crate::apis::EventSubscriptionApi {
    //     self.event_subscription_api.as_ref()
    // }

    pub fn external_task_api(self) -> &dyn crate::apis::ExternalTaskApi {
        self.external_task_api
    }

    // pub fn message_api(&self) -> &dyn crate::apis::MessageApi {
    //     self.message_api.as_ref()
    // }

    // pub fn metrics_api(&self) -> &dyn crate::apis::MetricsApi {
    //     self.metrics_api.as_ref()
    // }

    // pub fn process_definition_api(&self) -> &dyn crate::apis::ProcessDefinitionApi {
    //     self.process_definition_api.as_ref()
    // }

    // pub fn process_instance_api(&self) -> &dyn crate::apis::ProcessInstanceApi {
    //     self.process_instance_api.as_ref()
    // }

    // pub fn schema_log_api(&self) -> &dyn crate::apis::SchemaLogApi {
    //     self.schema_log_api.as_ref()
    // }

    // pub fn signal_api(&self) -> &dyn crate::apis::SignalApi {
    //     self.signal_api.as_ref()
    // }

    // pub fn task_api(&self) -> &dyn crate::apis::TaskApi {
    //     self.task_api.as_ref()
    // }

    // pub fn task_attachment_api(&self) -> &dyn crate::apis::TaskAttachmentApi {
    //     self.task_attachment_api.as_ref()
    // }

    // pub fn task_comment_api(&self) -> &dyn crate::apis::TaskCommentApi {
    //     self.task_comment_api.as_ref()
    // }

    // pub fn task_identity_link_api(&self) -> &dyn crate::apis::TaskIdentityLinkApi {
    //     self.task_identity_link_api.as_ref()
    // }

    // pub fn task_local_variable_api(&self) -> &dyn crate::apis::TaskLocalVariableApi {
    //     self.task_local_variable_api.as_ref()
    // }

    // pub fn task_variable_api(&self) -> &dyn crate::apis::TaskVariableApi {
    //     self.task_variable_api.as_ref()
    // }

    // pub fn version_api(&self) -> &dyn crate::apis::VersionApi {
    //     self.version_api.as_ref()
    // }
}

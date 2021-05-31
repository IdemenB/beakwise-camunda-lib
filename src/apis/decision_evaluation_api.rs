use async_trait::async_trait;
use std::borrow::Borrow;
use std::option::Option;
use std::sync::Arc;

use super::configuration;
use crate::errors::errors::CamundaClientError;
use crate::models::DecisionDto;
use crate::models::DecisionEvaluationDto;
use crate::utils::url_encode;

pub struct DecisionEvaluationApiClient {
    configuration: Arc<configuration::Configuration>,
}

impl DecisionEvaluationApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> DecisionEvaluationApiClient {
        DecisionEvaluationApiClient { configuration }
    }
}

#[async_trait]
pub trait DecisionEvaluationApi {
    async fn evaluate_decision(
        &self,
        key: &str,
        decision_evaluation_dto: DecisionEvaluationDto,
    ) -> Result<DecisionDto, CamundaClientError>;
}

#[async_trait]
impl DecisionEvaluationApi for DecisionEvaluationApiClient {
    async fn evaluate_decision(
        &self,
        key: &str,
        decision_evaluation_dto: DecisionEvaluationDto,
    ) -> Result<DecisionDto, CamundaClientError> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/decision-definition/key/{}/evaluate",
            self._config.base_path, key
        );
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&decision_evaluation_dto);

        // send request
        let resp = req_builder.send().await?;

        Ok(resp.error_for_status()?.json().await?)
    }
}

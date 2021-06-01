use crate::apis::urlencode;
use crate::models::DecisionDto;
use crate::models::DecisionEvaluationDto;

use super::configuration;
use std::borrow::Borrow;
use std::rc::Rc;

pub struct DecisionEvaluationApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl DecisionEvaluationApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> DecisionEvaluationApiClient {
        DecisionEvaluationApiClient { configuration }
    }
}

pub trait DecisionEvaluationApi {
    fn evaluate_decision(
        &self,
        key: &str,
        decision_evaluation_dto: DecisionEvaluationDto,
    ) -> Result<DecisionDto, crate::apis::CamundaClientError>;
}

impl DecisionEvaluationApi for DecisionEvaluationApiClient {
    fn evaluate_decision(
        &self,
        key: &str,
        decision_evaluation_dto: DecisionEvaluationDto,
    ) -> Result<DecisionDto, crate::apis::CamundaClientError> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/decision-definition/key/{}/evaluate",
            configuration.base_path,
            urlencode(key)
        );
        let mut resp_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            resp_builder = resp_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        resp_builder = resp_builder.json(&decision_evaluation_dto);

        // send respuest
        let resp = resp_builder.build()?;

        Ok(client.execute(resp)?.error_for_status()?.json()?)
    }
}

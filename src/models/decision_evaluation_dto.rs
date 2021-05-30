use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};

use super::VariableValueDto;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DecisionEvaluationDto {
    /// A JSON object containing variable key-value pairs.
    #[serde(rename = "variables", skip_serializing_if = "Option::is_none")]
    pub variables: Option<HashMap<String, crate::models::VariableValueDto>>,
}

impl DecisionEvaluationDto {
    pub fn new(variables: HashMap<String, VariableValueDto>) -> DecisionEvaluationDto {
        DecisionEvaluationDto {
            variables: Some(variables),
        }
    }
}

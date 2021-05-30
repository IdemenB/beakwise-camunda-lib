use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};

use super::VariableValueDto;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DecisionDto {
    /// A JSON object containing variable key-value pairs.
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<::std::collections::HashMap<String, VariableValueDto>>,
}

impl DecisionDto {
    pub fn new(result: HashMap<String, VariableValueDto>) -> DecisionDto {
        DecisionDto {
            result: Some(result),
        }
    }
}

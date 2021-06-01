/*
 * Camunda BPM REST API
 *
 * OpenApi Spec for Camunda BPM REST API.
 *
 * The version of the OpenAPI document: 7.13.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskEscalationDto {
    /// An escalation code that indicates the predefined escalation. It is used to identify the BPMN escalation handler.
    #[serde(rename = "escalationCode", skip_serializing_if = "Option::is_none")]
    pub escalation_code: Option<String>,
    /// A JSON object containing variable key-value pairs.
    #[serde(rename = "variables", skip_serializing_if = "Option::is_none")]
    pub variables: Option<::std::collections::HashMap<String, crate::models::VariableValueDto>>,
}

impl TaskEscalationDto {
    pub fn new() -> TaskEscalationDto {
        TaskEscalationDto {
            escalation_code: None,
            variables: None,
        }
    }
}



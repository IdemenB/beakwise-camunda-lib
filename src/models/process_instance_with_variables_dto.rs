/*
 * Camunda BPM REST API
 *
 * OpenApi Spec for Camunda BPM REST API.
 *
 * The version of the OpenAPI document: 7.13.0
 *
 * Generated by: https://openapi-generator.tech
 */
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProcessInstanceWithVariablesDto {
    /// The id of the process instance.
    #[serde(rename = "variables", skip_serializing_if = "Option::is_none")]
    pub variables: Option<::std::collections::HashMap<String, crate::models::VariableValueDto>>,
    /// The id of the process instance.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The id of the process definition that this process instance belongs to.
    #[serde(rename = "definitionId", skip_serializing_if = "Option::is_none")]
    pub definition_id: Option<String>,
    /// The business key of the process instance.
    #[serde(rename = "businessKey", skip_serializing_if = "Option::is_none")]
    pub business_key: Option<String>,
    /// The id of the case instance associated with the process instance.
    #[serde(rename = "caseInstanceId", skip_serializing_if = "Option::is_none")]
    pub case_instance_id: Option<String>,
    /// A flag indicating whether the process instance has ended or not. Deprecated: will always be false!
    #[serde(rename = "ended", skip_serializing_if = "Option::is_none")]
    pub ended: Option<bool>,
    /// A flag indicating whether the process instance is suspended or not.
    #[serde(rename = "suspended", skip_serializing_if = "Option::is_none")]
    pub suspended: Option<bool>,
    /// The tenant id of the process instance.
    #[serde(rename = "tenantId", skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    /// The links associated to this resource, with `method`, `href` and `rel`.
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<crate::models::AtomLink>>,
}

impl ProcessInstanceWithVariablesDto {
    pub fn new() -> ProcessInstanceWithVariablesDto {
        ProcessInstanceWithVariablesDto {
            variables: None,
            id: None,
            definition_id: None,
            business_key: None,
            case_instance_id: None,
            ended: None,
            suspended: None,
            tenant_id: None,
            links: None,
        }
    }
}

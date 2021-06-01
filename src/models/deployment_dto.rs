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
pub struct DeploymentDto {
    /// The id of the deployment.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The tenant id of the deployment.
    #[serde(rename = "tenantId", skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    /// The time when the deployment was created.
    #[serde(rename = "deploymentTime", skip_serializing_if = "Option::is_none")]
    pub deployment_time: Option<String>,
    /// The source of the deployment.
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// The name of the deployment.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The links associated to this resource, with `method`, `href` and `rel`.
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<crate::models::AtomLink>>,
}

impl DeploymentDto {
    pub fn new() -> DeploymentDto {
        DeploymentDto {
            id: None,
            tenant_id: None,
            deployment_time: None,
            source: None,
            name: None,
            links: None,
        }
    }
}



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
pub struct MultiFormDeploymentDto {
    /// The tenant id for the deployment to be created.
    #[serde(rename = "tenant-id", skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    /// The source for the deployment to be created.
    #[serde(rename = "deployment-source", skip_serializing_if = "Option::is_none")]
    pub deployment_source: Option<String>,
    /// A flag indicating whether the process engine should perform duplicate checking on a per-resource basis. If set to true, only those resources that have actually changed are deployed. Checks are made against resources included previous deployments of the same name and only against the latest versions of those resources. If set to true, the option enable-duplicate-filtering is overridden and set to true.
    #[serde(rename = "deploy-changed-only", skip_serializing_if = "Option::is_none")]
    pub deploy_changed_only: Option<bool>,
    /// A flag indicating whether the process engine should perform duplicate checking for the deployment or not. This allows you to check if a deployment with the same name and the same resouces already exists and if true, not create a new deployment but instead return the existing deployment. The default value is false.
    #[serde(rename = "enable-duplicate-filtering", skip_serializing_if = "Option::is_none")]
    pub enable_duplicate_filtering: Option<bool>,
    /// The name for the deployment to be created.
    #[serde(rename = "deployment-name", skip_serializing_if = "Option::is_none")]
    pub deployment_name: Option<String>,
    /// The binary data to create the deployment resource. It is possible to have more than one form part with different form part names for the binary data to create a deployment.
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<std::path::PathBuf>,
}

impl MultiFormDeploymentDto {
    pub fn new() -> MultiFormDeploymentDto {
        MultiFormDeploymentDto {
            tenant_id: None,
            deployment_source: None,
            deploy_changed_only: None,
            enable_duplicate_filtering: None,
            deployment_name: None,
            data: None,
        }
    }
}



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
pub struct MetricsResultDto {
    /// The current sum (count) for the selected metric.
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<i64>,
}

impl MetricsResultDto {
    pub fn new() -> MetricsResultDto {
        MetricsResultDto { result: None }
    }
}

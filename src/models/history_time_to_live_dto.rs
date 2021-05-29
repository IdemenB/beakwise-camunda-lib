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
pub struct HistoryTimeToLiveDto {
    /// New value for historyTimeToLive field of the definition. Can be `null`. Can not be negative.
    #[serde(rename = "historyTimeToLive", skip_serializing_if = "Option::is_none")]
    pub history_time_to_live: Option<i32>,
}

impl HistoryTimeToLiveDto {
    pub fn new() -> HistoryTimeToLiveDto {
        HistoryTimeToLiveDto {
            history_time_to_live: None,
        }
    }
}

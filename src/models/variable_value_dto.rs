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
pub struct VariableValueDto {
    /// The variable's value. Value differs depending on the variable's type and on the deserializeValues parameter.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
    /// The value type of the variable.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    /// A JSON object containing additional, value-type-dependent properties. For serialized variables of type Object, the following properties can be provided:  * `objectTypeName`: A string representation of the object's type name. * `serializationDataFormat`: The serialization format used to store the variable.  For serialized variables of type File, the following properties can be provided:  * `filename`: The name of the file. This is not the variable name but the name that will be used when downloading the file again. * `mimetype`: The MIME type of the file that is being uploaded. * `encoding`: The encoding of the file that is being uploaded.
    #[serde(rename = "valueInfo", skip_serializing_if = "Option::is_none")]
    pub value_info: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl VariableValueDto {
    pub fn new() -> VariableValueDto {
        VariableValueDto {
            value: None,
            _type: None,
            value_info: None,
        }
    }
}

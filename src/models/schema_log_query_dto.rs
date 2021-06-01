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
pub struct SchemaLogQueryDto {
    /// The version of the schema.
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// A JSON array of criteria to sort the result by. Each element of the array is                       a JSON object that specifies one ordering. The position in the array                       identifies the rank of an ordering, i.e., whether it is primary, secondary,                       etc. 
    #[serde(rename = "sorting", skip_serializing_if = "Option::is_none")]
    pub sorting: Option<Vec<crate::models::SchemaLogQueryDtoSorting>>,
}

impl SchemaLogQueryDto {
    pub fn new() -> SchemaLogQueryDto {
        SchemaLogQueryDto {
            version: None,
            sorting: None,
        }
    }
}



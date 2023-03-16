/*
 * Headless Batch Engine
 *
 * A Java client JAR is available for use with the group ID 'com.liferay', artifact ID 'com.liferay.headless.batch.engine.client', and version '1.0.11'.
 *
 * The version of the OpenAPI document: v1.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ExportTask {
    /// The item class name for which data will be exported in batch.
    #[serde(rename = "className", skip_serializing_if = "Option::is_none")]
    pub class_name: Option<String>,
    /// The file content type.
    #[serde(rename = "contentType", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// The end time of export task operation.
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// The error message in case of export task's failed execution.
    #[serde(rename = "errorMessage", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// The status of export task's execution.
    #[serde(rename = "executeStatus", skip_serializing_if = "Option::is_none")]
    pub execute_status: Option<ExecuteStatus>,
    /// The optional external key of this account.
    #[serde(rename = "externalReferenceCode", skip_serializing_if = "Option::is_none")]
    pub external_reference_code: Option<String>,
    /// The task's ID.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Number of items processed by export task opeartion.
    #[serde(rename = "processedItemsCount", skip_serializing_if = "Option::is_none")]
    pub processed_items_count: Option<i32>,
    /// The start time of export task operation.
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// Total number of items that will be processed by export task operation.
    #[serde(rename = "totalItemsCount", skip_serializing_if = "Option::is_none")]
    pub total_items_count: Option<i32>,
    #[serde(rename = "x-class-name", skip_serializing_if = "Option::is_none")]
    pub x_class_name: Option<String>,
}

impl ExportTask {
    pub fn new() -> ExportTask {
        ExportTask {
            class_name: None,
            content_type: None,
            end_time: None,
            error_message: None,
            execute_status: None,
            external_reference_code: None,
            id: None,
            processed_items_count: None,
            start_time: None,
            total_items_count: None,
            x_class_name: None,
        }
    }
}

/// The status of export task's execution.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ExecuteStatus {
    #[serde(rename = "COMPLETED")]
    Completed,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "INITIAL")]
    Initial,
    #[serde(rename = "STARTED")]
    Started,
}

impl Default for ExecuteStatus {
    fn default() -> ExecuteStatus {
        Self::Completed
    }
}


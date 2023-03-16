/*
 * Object
 *
 * A Java client JAR is available for use with the group ID 'com.liferay', artifact ID 'com.liferay.object.admin.rest.client', and version '1.0.45'.
 *
 * The version of the OpenAPI document: v1.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ObjectViewFilterColumn {
    #[serde(rename = "filterType", skip_serializing_if = "Option::is_none")]
    pub filter_type: Option<FilterType>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "json", skip_serializing_if = "Option::is_none")]
    pub json: Option<String>,
    #[serde(rename = "objectFieldName", skip_serializing_if = "Option::is_none")]
    pub object_field_name: Option<String>,
    #[serde(rename = "valueSummary", skip_serializing_if = "Option::is_none")]
    pub value_summary: Option<String>,
    #[serde(rename = "x-class-name", skip_serializing_if = "Option::is_none")]
    pub x_class_name: Option<String>,
}

impl ObjectViewFilterColumn {
    pub fn new() -> ObjectViewFilterColumn {
        ObjectViewFilterColumn {
            filter_type: None,
            id: None,
            json: None,
            object_field_name: None,
            value_summary: None,
            x_class_name: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FilterType {
    #[serde(rename = "excludes")]
    Excludes,
    #[serde(rename = "includes")]
    Includes,
}

impl Default for FilterType {
    fn default() -> FilterType {
        Self::Excludes
    }
}


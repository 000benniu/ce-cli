/*
 * List Type
 *
 * A Java client JAR is available for use with the group ID 'com.liferay', artifact ID 'om.liferay.headless.admin.list.type.client', and version '1.0.0'.. A Java client JAR is available for use with the group ID 'com.liferay', artifact ID 'com.liferay.headless.admin.list.type.client', and version '1.0.11'.
 *
 * The version of the OpenAPI document: v1.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PageListTypeDefinition {
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<crate::models::ListTypeDefinition>>,
    #[serde(rename = "totalCount", skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    #[serde(rename = "lastPage", skip_serializing_if = "Option::is_none")]
    pub last_page: Option<i64>,
    #[serde(rename = "facets", skip_serializing_if = "Option::is_none")]
    pub facets: Option<Vec<crate::models::Facet>>,
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(rename = "pageSize", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[serde(rename = "actions", skip_serializing_if = "Option::is_none")]
    pub actions: Option<::std::collections::HashMap<String, ::std::collections::HashMap<String, String>>>,
}

impl PageListTypeDefinition {
    pub fn new() -> PageListTypeDefinition {
        PageListTypeDefinition {
            items: None,
            total_count: None,
            last_page: None,
            facets: None,
            page: None,
            page_size: None,
            actions: None,
        }
    }
}



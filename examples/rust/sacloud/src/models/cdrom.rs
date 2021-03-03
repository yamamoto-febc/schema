/*
 * SAKURA Cloud APIs
 *
 * This is a definitions for SAKURA Cloud APIs.
 *
 * The version of the OpenAPI document: 0.0.1
 * Contact: sacloud.users@gmail.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cdrom {
    #[serde(rename = "ID")]
    pub ID: crate::models::Id,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Scope")]
    pub scope: crate::models::Scope,
    #[serde(rename = "SizeMB")]
    pub size_mb: crate::models::CdromSizes,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Tags")]
    pub tags: Vec<String>,
    #[serde(rename = "Availability")]
    pub availability: String,
    #[serde(rename = "CreatedAt")]
    pub created_at: String,
    #[serde(rename = "DisplayOrder")]
    pub display_order: i32,
    #[serde(rename = "Icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<crate::models::IconView>,
    #[serde(rename = "ModifiedAt")]
    pub modified_at: String,
    #[serde(rename = "StorageClass")]
    pub storage_class: String,
}

impl Cdrom {
    pub fn new(ID: crate::models::Id, name: String, scope: crate::models::Scope, size_mb: crate::models::CdromSizes, description: String, tags: Vec<String>, availability: String, created_at: String, display_order: i32, modified_at: String, storage_class: String) -> Cdrom {
        Cdrom {
            ID,
            name,
            scope,
            size_mb,
            description,
            tags,
            availability,
            created_at,
            display_order,
            icon: None,
            modified_at,
            storage_class,
        }
    }
}


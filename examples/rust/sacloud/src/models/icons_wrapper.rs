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
pub struct IconsWrapper {
    #[serde(rename = "Icons")]
    pub icons: Vec<crate::models::Icon>,
}

impl IconsWrapper {
    pub fn new(icons: Vec<crate::models::Icon>) -> IconsWrapper {
        IconsWrapper {
            icons,
        }
    }
}


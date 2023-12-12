use serde::{Deserialize, Serialize};
use super::link::Link;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PermissionResource {
    resource: String,
    pub id: String,
    pub description: String,
    pub granted: bool,

    #[serde(rename = "_links")]
    links: std::collections::HashMap<String, Link>    
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PermissionsListResource {
    count: i32,
    #[serde(rename = "_embedded")]
    pub embedded: PermissionsEmbeddedResource,
    #[serde(rename = "_links")]
    links: std::collections::HashMap<String, Link>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PermissionsEmbeddedResource {
    pub permissions: Vec<PermissionResource>
}
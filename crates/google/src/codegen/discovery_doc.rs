#![allow(unused)]
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscoveryItem {
    pub kind: String,
    #[serde(default)]
    pub id: String,
    pub name: String,
    pub version: String,
    pub title: String,
    #[serde(default)]
    pub description: String,
    pub discovery_rest_url: String,
    pub icons: Icons,
    #[serde(default)]
    pub documentation_link: String,
    pub preferred: bool,
}

impl DiscoveryItem {
    pub async fn get_discovery_document(&self, client: &reqwest::Client) -> crate::error::Result<DiscoveryDocument> {
        let response = client.get(&self.discovery_rest_url).send().await?;
        let json = response.json::<serde_json::Value>().await?;
        let doc = serde_json::from_value(json)?;
        Ok(doc)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Icons {
    pub x16: String,
    pub x32: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscoveryDocument {
    pub kind: String,
    pub discovery_version: String,
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub canonical_name: String,
    pub version: String,
    pub revision: String,
    pub title: String,
    #[serde(default)]
    pub description: String,
    pub icons: Icons,
    pub documentation_link: String,
    #[serde(default)]
    pub labels: Vec<String>,
    pub protocol: String,
    pub root_url: String,
    pub service_path: String,
    pub batch_path: String,
    #[serde(default)]
    pub parameters: HashMap<String, JsonSchema>,
    pub auth: Option<Auth>,
    #[serde(default)]
    pub features: Vec<String>,
    #[serde(default)]
    pub schemas: HashMap<String, JsonSchema>,
    #[serde(default)]
    pub methods: HashMap<String, Method>,
    #[serde(default)]
    pub resources: HashMap<String, Resource>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Auth {
    #[serde(default)]
    oauth2: OAuth2
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OAuth2 {
    #[serde(default)]
    pub scopes: HashMap<String, Scope>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Scope {
    #[serde(default)]
    pub description: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonSchema {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub type_: String,
    #[serde(default)]
    pub ref_: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub default: String,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub deprecated: bool,
    #[serde(default)]
    pub format: String,
    #[serde(default)]
    pub pattern: String,
    #[serde(default)]
    pub minimum: String,
    #[serde(default)]
    pub maximum: String,
    #[serde(default)]
    pub enum_: Vec<String>,
    #[serde(default)]
    pub enum_descriptions: Vec<String>,
    #[serde(default)]
    pub enum_deprecated: Vec<bool>,
    #[serde(default)]
    pub repeated: bool,
    #[serde(default)]
    pub location: String,
    #[serde(default)]
    pub properties: HashMap<String, JsonSchema>,
    pub additional_properties: Option<Box<JsonSchema>>,
    pub items: Option<Box<JsonSchema>>,
    #[serde(default)]
    pub annotations: Annotations
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Annotations {
    #[serde(default)]
    pub required: Vec<String>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Method {
    pub id: String,
    pub path: String,
    pub http_method: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub deprecated: bool,
    #[serde(default)]
    pub parameters: HashMap<String, JsonSchema>,
    #[serde(default)]
    pub parameter_order: Vec<String>,
    pub request: Option<Reference>,
    pub response: Option<Reference>,
    #[serde(default)]
    pub scopes: Vec<String>,
    #[serde(default)]
    pub supports_media_download: bool,
    #[serde(default)]
    pub supports_media_upload: bool,
    pub media_upload: Option<MediaUpload>,
    #[serde(default)]
    pub supports_subscription: bool
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reference {
    #[serde(default)]
    pub ref_: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaUpload {
    #[serde(default)]
    pub accept: Vec<String>,
    #[serde(default)]
    pub max_size: String,
    #[serde(default)]
    pub protocols: MediaProtocols
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaProtocols {
    pub simple: Option<Protocol>,
    pub resumable: Option<Protocol>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Protocol {
    pub multipart: bool,
    pub path: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    #[serde(default)]
    pub methods: HashMap<String, Method>,
    #[serde(default)]
    pub deprecated: bool,
    #[serde(default)]
    pub resources: HashMap<String, Resource>
}

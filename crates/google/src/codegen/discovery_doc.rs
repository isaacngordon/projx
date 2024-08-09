#![allow(unused)]
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscoveryItem {
    pub kind: String,
    pub id: String,
    pub name: String,
    pub version: String,
    pub title: String,
    pub description: String,
    pub discovery_rest_url: String,
    pub icons: Icons,
    pub documentation_link: String,
    pub preferred: bool,
}

impl DiscoveryItem {
    pub async fn get_discovery_document(&self) -> crate::error::Result<DiscoveryDocument> {
        let client = reqwest::Client::new();
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
    pub canonical_name: String,
    pub version: String,
    pub revision: String,
    pub title: String,
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
    pub auth: Auth,
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
    pub description: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonSchema {
    pub id: String,
    pub type_: String,
    pub ref_: String,
    pub description: String,
    pub default: String,
    pub required: bool,
    pub deprecated: bool,
    pub format: String,
    pub pattern: String,
    pub minimum: String,
    pub maximum: String,
    #[serde(default)]
    pub enum_: Vec<String>,
    #[serde(default)]
    pub enum_descriptions: Vec<String>,
    #[serde(default)]
    pub enum_deprecated: Vec<bool>,
    pub repeated: bool,
    pub location: String,
    #[serde(default)]
    pub properties: HashMap<String, JsonSchema>,
    pub additional_properties: Box<JsonSchema>,
    pub items: Box<JsonSchema>,
    pub annotations: Annotations
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Annotations {
    pub required: Vec<String>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Method {
    pub id: String,
    pub path: String,
    pub http_method: String,
    pub description: String,
    pub deprecated: bool,
    #[serde(default)]
    pub parameters: HashMap<String, JsonSchema>,
    pub parameter_order: Vec<String>,
    pub request: Reference,
    pub response: Reference,
    pub scopes: Vec<String>,
    pub supports_media_download: bool,
    pub supports_media_upload: bool,
    pub media_upload: MediaUpload,
    pub supports_subscription: bool
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reference {
    pub ref_: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaUpload {
    pub accept: Vec<String>,
    pub max_size: String,
    pub protocols: MediaProtocols
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaProtocols {
    pub simple: Protocol,
    pub resumable: Protocol
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
    pub deprecated: bool,
    #[serde(default)]
    pub resources: HashMap<String, Resource>
}

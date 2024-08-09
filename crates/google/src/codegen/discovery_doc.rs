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
    pub documentation_link: Option<String>,
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
    pub labels: Vec<String>,
    pub protocol: String,
    pub root_url: String,
    pub service_path: String,
    pub batch_path: String,
    pub parameters: HashMap<String, JsonSchema>,
    pub auth: Auth,
    pub features: Vec<String>,
    pub schemas: HashMap<String, JsonSchema>,
    pub methods: HashMap<String, Method>,
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
    pub id: Option<String>,
    pub type_: Option<String>,
    pub ref_: Option<String>,
    pub description: Option<String>,
    pub default: Option<String>,
    pub required: Option<bool>,
    pub deprecated: Option<bool>,
    pub format: Option<String>,
    pub pattern: Option<String>,
    pub minimum: Option<String>,
    pub maximum: Option<String>,
    pub enum_: Option<Vec<String>>,
    pub enum_descriptions: Option<Vec<String>>,
    pub enum_deprecated: Option<Vec<bool>>,
    pub repeated: Option<bool>,
    pub location: Option<String>,
    pub properties: Option<HashMap<String, JsonSchema>>,
    pub additional_properties: Option<Box<JsonSchema>>,
    pub items: Option<Box<JsonSchema>>,
    pub annotations: Option<Annotations>
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
    pub deprecated: Option<bool>,
    pub parameters: HashMap<String, JsonSchema>,
    pub parameter_order: Vec<String>,
    pub request: Option<Reference>,
    pub response: Option<Reference>,
    pub scopes: Vec<String>,
    pub supports_media_download: Option<bool>,
    pub supports_media_upload: Option<bool>,
    pub media_upload: Option<MediaUpload>,
    pub supports_subscription: Option<bool>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reference {
    pub ref_: Option<String>
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
    pub methods: HashMap<String, Method>,
    pub deprecated: Option<bool>,
    pub resources: HashMap<String, Resource>
}

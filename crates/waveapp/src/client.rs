use crate::error::{Error, Result};
use cynic::{
    serde::{Deserialize, Serialize},
    GraphQlResponse,
};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::fmt;

const GRAPHQL_ENDPOINT: &str = "https://gql.waveapps.com/graphql/public";
pub const SCHEMA: &str = "./schemas/wave_api_schema_20240728.graphql";

#[cynic::schema("waveapp")]
pub mod schema {}

#[derive(Debug, Clone)]
pub struct WaveAppClient {
    url: String,
    headers: HeaderMap,
    business_id: Option<String>,
}

impl Default for WaveAppClient {
    fn default() -> Self {
        let wave_tkn = std::env::var("WAVEAPP_FULL_ACCESS_TOKEN")
            .expect("Could not find env variable WAVEAPP_FULL_ACCESS_TOKEN");

        let headers = vec![
            format!("Authorization: Bearer {}", wave_tkn),
            "Content-Type: application/json".to_string(),
        ];

        Self {
            url: GRAPHQL_ENDPOINT.to_owned(),
            headers: headers
                .iter()
                .map(|x| {
                    let (name, value) = x.split_at(x.find(':').unwrap());
                    let name = HeaderName::from_bytes(name.trim().as_bytes()).unwrap();
                    let value = HeaderValue::from_bytes(value[1..].trim().as_bytes()).unwrap();
                    (name, value)
                })
                .collect::<HeaderMap>(),
            business_id: None,
        }
    }
}

impl WaveAppClient {
    pub fn new(business_id: String) -> Self {
        Self {
            url: Default::default(),
            headers: Default::default(),
            business_id: Some(business_id),
        }
    }

    pub async fn query_raw<T>(
        &self,
        operation: impl Serialize + Sized,
    ) -> Result<GraphQlResponse<T>>
    where
        T: for<'de> Deserialize<'de> + fmt::Debug,
    {
        let req = reqwest::Client::new()
            .post(self.url.clone())
            .headers(self.headers.clone())
            .json(&operation)
            .send();

        let res = req.await?;
        let status_code = res.status();

        let data = res.text().await?;

        if status_code.is_success() {
            // Use a generic lifetime `'de` for deserialization
            let result: GraphQlResponse<T> = serde_json::from_str(&data)?;
            Ok(result)
        } else {
            Err(Error::GraphQLCustomError {
                url: self.url.clone(),
                status_code: status_code.as_u16(),
                message: data,
            })
        }
    }
}

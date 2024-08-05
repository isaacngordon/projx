use crate::error::{Error, Result};
use cynic::{
    serde::{Deserialize, Serialize},
    GraphQlResponse,
};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use schema::__fields::InputError::message;
use std::fmt;

const GRAPHQL_ENDPOINT: &str = "https://gql.waveapps.com/graphql/public";
pub const SCHEMA: &str = "./schemas/wave_api_schema_20240728.graphql";

#[cynic::schema("waveapp")]
pub mod schema {}

#[derive(Debug, Clone)]
pub struct WaveAppClient {
    url: String,
}

pub enum WaveAppOperation {
    Query(schema::Query),
    Mutation(schema::Mutation),
}

pub struct WaveAppPayload {
    pub operation: WaveAppOperation,
    pub variables: Option<String>,
}

pub trait WaveAppRequest: Serialize {
    fn to_payload(&self) -> WaveAppPayload;
}

pub trait WaveAppResponse {
    fn from_response(res: reqwest::Response) -> Result<Self>
    where
        Self: Sized;
}

impl Default for WaveAppClient {
    fn default() -> Self {
        Self {
            url: GRAPHQL_ENDPOINT.to_owned(),
        }
    }
}

impl WaveAppClient {
    pub async fn query_raw<T>(
        &self,
        operation: impl Serialize + Sized,
    ) -> Result<GraphQlResponse<T>>
    where
        T: for<'de> Deserialize<'de> + fmt::Debug,
    {
        let wave_tkn = std::env::var("WAVEAPP_FULL_ACCESS_TOKEN")
            .expect("Could not find env variable WAVEAPP_FULL_ACCESS_TOKEN");
        let headers = vec![
            format!("Authorization: Bearer {}", wave_tkn),
            "Content-Type: application/json".to_string(),
        ];

        let x = reqwest::Client::new()
            .post(self.url.clone())
            .headers(
                headers
                    .iter()
                    .map(|x| {
                        let (name, value) = x.split_at(x.find(':').unwrap());
                        let name = HeaderName::from_bytes(name.trim().as_bytes()).unwrap();
                        let value = HeaderValue::from_bytes(value[1..].trim().as_bytes()).unwrap();
                        (name, value)
                    })
                    .collect::<HeaderMap>(),
            )
            .json(&operation)
            .send();

        let res = x.await.map_err(|e| Error::CustomError(e.to_string()))?;
        let status_code = res.status();

        let data = res
            .text()
            .await
            .map_err(|e| Error::CustomError(e.to_string()))?;

        if status_code.is_success() {
            // Use a generic lifetime `'de` for deserialization
            let result: GraphQlResponse<T> = serde_json::from_str(&data)?;
            Ok(result)
        } else {
            Err(Error::GraphQL {
                url: self.url.clone(),
                status_code: status_code.as_u16(),
                message: data,
            })
        }
    }
}

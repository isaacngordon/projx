use cynic::serde::{Deserialize, Serialize};
use reqwest;
use std::fmt;

const GRAPHQL_ENDPOINT: &str = "https://gql.waveapps.com/graphql/public";
pub const SCHEMA: &str =  "./schemas/wave_api_schema_20240728.graphql";

mod schema {
    cynic::use_schema!("./schemas/wave_api_schema_20240728.graphql");
}

#[derive(cynic::QueryFragment)]
#[cynic(schema_path = "./schemas/wave_api_schema_20240728.graphql")]
struct Business {
    id: Option<cynic::Id>
}

pub struct WaveAppClient {
    url: String,
}

pub struct WaveAppPayload {
    operation: String,
    variables: String,
}

impl fmt::Display for WaveAppPayload {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "WaveAppPayload {{ operation: {}, variables: {} }}",
            self.operation, self.variables
        )
    }
}

impl Default for WaveAppClient {
    fn default() -> Self {
        Self {
            url: GRAPHQL_ENDPOINT.to_owned(),
        }
    }
}

impl WaveAppClient {
    fn query(self, payload: WaveAppPayload) {
        let wave_tkn = std::env::var("WAVEAPP_FULL_ACCESS_TOKEN")
            .expect("Could not find env variable WAVEAPP_FULL_ACCESS_TOKEN");
        let headers = vec![
            format!("Authorization: Bearer {}", wave_tkn),
            "Content-Type: application/json".to_string(),
        ];
        let payload = serde_json::to_string(&payload)?;
        let x = reqwest::Client::new()
            .post(self.url)
            .json(&operation)
            .send();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wave_app_client_query() {
        // Create a test instance of WaveAppClient
        let client = WaveAppClient::default();

        // Create a test payload
        let payload = WaveAppPayload {
            operation: String::from("test_operation"),
            variables: String::from("test_variables"),
        };

        // Call the query method
        client.query(payload);

        // Add assertions here to verify the behavior of the query method
        // For example, you can assert that the request was successful or check the response data
    }

    #[test]
    fn test_get_a_business() {
        let client = WaveAppClient::default();
    }
}

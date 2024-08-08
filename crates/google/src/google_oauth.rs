use std::fs;

use jsonwebtoken::{encode, EncodingKey, Header};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Deserialize)]
struct ServiceAccount {
    #[serde(rename = "type")]
    account_type: String,
    private_key: String,
    client_email: String,
    token_uri: String,
}

#[derive(Debug, Serialize)]
struct Claims {
    iss: String,
    scope: String,
    aud: String,
    exp: usize,
    iat: usize,
}

pub struct GoogleOAuth {
    client: Client,
    service_account: ServiceAccount,
}

impl GoogleOAuth {
    pub fn new(service_account_path: &str) -> Self {
        let service_account: ServiceAccount =
            serde_json::from_str(&fs::read_to_string(service_account_path).unwrap()).unwrap();
        let client = Client::new();

        GoogleOAuth {
            client,
            service_account,
        }
    }

    pub async fn obtain_access_token(
        &self,
        scopes: &[&str],
    ) -> Result<String, Box<dyn std::error::Error>> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as usize;
        let exp = now + 3600; // Token valid for 1 hour
        let claims = Claims {
            iss: self.service_account.client_email.clone(),
            scope: scopes.join(" "),
            aud: self.service_account.token_uri.clone(),
            exp,
            iat: now,
        };

        let header = Header::new(jsonwebtoken::Algorithm::RS256);
        let encoding_key = EncodingKey::from_rsa_pem(self.service_account.private_key.as_bytes())?;
        let jwt = encode(&header, &claims, &encoding_key)?;

        let response = self
            .client
            .post(&self.service_account.token_uri)
            .form(&[
                ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
                ("assertion", &jwt),
            ])
            .send()
            .await?;

        let json: HashMap<String, String> = response.json().await?;
        let access_token = json
            .get("access_token")
            .ok_or("Access token not found")?
            .clone();

        Ok(access_token)
    }
}

mod tests {

    use super::*;

    #[tokio::test]
    async fn test_auth_works() {
        let oauth = GoogleOAuth::new("crates/google/config/service-account.json");
        let scopes = vec!["https://www.googleapis.com/auth/cloud-platform"]; // Define your scopes here

        match oauth.obtain_access_token(&scopes).await {
            Ok(token) => println!("Access Token: {}", token),

            Err(e) => eprintln!("Error obtaining access token: {}", e),
        }
    }
}

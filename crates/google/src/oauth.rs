#![allow(unused)]
use jsonwebtoken::{encode, EncodingKey, Header};
use reqwest::{Client, ClientBuilder};
use serde::{Deserialize, Serialize};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Deserialize)]
struct ServiceAccount {
    #[serde(rename = "type")]
    account_type: String,
    private_key: String,
    client_email: String,
    token_uri: String,
}

#[derive(Debug, Deserialize)]
pub struct AccessToken {
    access_token: String,
    token_type: String,
    expires_in: usize,
}

#[derive(Debug, Serialize)]
struct Claims {
    iss: String,
    scope: String,
    aud: String,
    exp: String, // Changed from usize to String
    iat: usize,
}

pub struct ServiceAccountOAuthManager {
    client: Client,
    service_account: ServiceAccount,
}

impl ServiceAccountOAuthManager {
    pub fn new(service_account_path: &str) -> Self {
        let service_account: ServiceAccount =
            serde_json::from_str(&fs::read_to_string(service_account_path).unwrap()).unwrap();
        let client = ClientBuilder::new()
            // Following redirects opens the client up to SSRF vulnerabilities.
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .unwrap();

        ServiceAccountOAuthManager {
            client,
            service_account,
        }
    }

    pub async fn obtain_access_token(&self, scopes: &[&str]) -> crate::error::Result<AccessToken> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as usize;
        let exp = (now + 3600).to_string(); // Convert to String
        let claims = Claims {
            iss: self.service_account.client_email.clone(),
            scope: scopes.join(" "),
            aud: self.service_account.token_uri.clone(),
            exp,
            iat: now,
        };

        let header = Header::new(jsonwebtoken::Algorithm::RS256);
        let encoding_key = EncodingKey::from_rsa_pem(self.service_account.private_key.as_bytes())?;
        let jwt: String = encode(&header, &claims, &encoding_key)?;

        let response = self
            .client
            .post(&self.service_account.token_uri)
            .form(&[
                ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
                ("assertion", &jwt),
            ])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(crate::error::Error::CustomError(format!(
                "HTTP status not success: {}",
                response.text().await?
            )));
        }

        let access_token: AccessToken = response.json().await?;
        Ok(access_token)
    }
}

#[cfg(test)]
mod googl_oauth_tests {
    use super::ServiceAccountOAuthManager;

    #[tokio::test]
    async fn test_auth_works() -> Result<(), crate::error::Error> {
        let oauth = ServiceAccountOAuthManager::new("./config/google_service_account.json");
        let scopes = vec!["https://www.googleapis.com/auth/calendar"]; // Define your scopes here

        let res = oauth.obtain_access_token(&scopes).await?;
        println!(
            "\n\n[[[   Access token expires in {} seconds   ]]]\n\n",
            res.expires_in
        );
        Ok(())
    }
}

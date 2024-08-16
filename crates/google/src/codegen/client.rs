use oauth2::AccessToken;

use super::utils::discover_specific_apis;
use super::DiscoveryDocument;
use super::traits::GoogleServiceClient;


pub struct CalendarClient {
    client: reqwest::Client,
    access_token: AccessToken,
    scopes: Vec<String>,
}

impl GoogleServiceClient for CalendarClient {
    fn set_access_token(&mut self, token: AccessToken) {
        self.access_token = token;
    }

    fn get_scopes(&self) -> Vec<String> {
        self.scopes.clone()
    }

    async fn discover(&self) -> crate::error::Result<DiscoveryDocument> {
        // Implementation to fetch and parse the discovery document
        let response= discover_specific_apis(&self.client, vec!["calendar".to_string()]).await?;
        let doc = response[0].get_discovery_document(&self.client).await?;
        Ok(doc)
    }
}
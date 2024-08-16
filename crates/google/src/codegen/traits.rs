use std::thread::AccessError;

use oauth2::AccessToken;

use super::DiscoveryDocument;

pub trait GoogleServiceClient {
    fn set_access_token(&mut self, token: AccessToken);
    /// Returns the set of distinct scopes for any and all method and resources
    fn get_scopes(&self) -> Vec<String>;  
    async fn discover(&self) -> crate::error::Result<DiscoveryDocument>;
}
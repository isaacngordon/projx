mod keep {
    use crate::oauth::AccessToken;

    pub struct keepClient {
        client: reqwest::Client,
        access_token: String,
        scopes: Vec<String>,
        root_url: String,
        service_path: String,
        version: String,
    }
    impl keepClient {
        pub fn new(access_token: AccessToken, scopes: Vec<String>) -> Self {
            Self {
                client: reqwest::Client::new(),
                access_token,
                scopes,
                ..Default::default()
            }
        }
    }
    impl Default for keepClient {
        #[doc = r" Use default service account to get and auth token for the service"]
        fn default() -> Self {
            Self {
                client: reqwest::Client::new(),
                access_token: AccessToken::default(),
                scopes: Self::get_scopes(),
                root_url: "https://keep.googleapis.com/".to_string(),
                service_path: "".to_string(),
                version: "v1".to_string(),
            }
        }
    }
    impl GoogleServiceClient for keepClient {
        fn set_access_token(&mut self, token: AccessToken) {
            self.access_token = token;
        }
        fn get_scopes(&self) -> Vec<String> {
            vec![
                "https://www.googleapis.com/auth/keep",
                "https://www.googleapis.com/auth/keep.readonly",
            ]
        }
        fn discover(&self) -> crate::error::Result<DiscoveryDocument> {
            let url = format!(
                "{}discovery/v1/apis/{}/{}",
                self.root_url,
                keepClient::API_NAME,
                self.version
            );
            let resp = self
                .client
                .get(&url)
                .bearer_auth(&self.access_token)
                .send()?;
            let doc = resp.json::<DiscoveryDocument>()?;
            Ok(doc)
        }
    }
}

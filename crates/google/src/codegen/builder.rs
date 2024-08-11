use crate::error::Result;

use super::{discover_specific_apis, DiscoveryDocument};

pub struct GoogleServicesAPIBuilder {
    client: reqwest::Client,
    services: Vec<String>,
}

impl Default for GoogleServicesAPIBuilder {
    fn default() -> Self {
        GoogleServicesAPIBuilder {
            client: reqwest::Client::new(),
            services: vec![],
        }
    }
}

impl GoogleServicesAPIBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_service_api(self, service: &str) -> Self {
        let mut services = self.services;
        services.push(service.to_string());
        GoogleServicesAPIBuilder {
            client: self.client,
            services,
        }
    }

    pub async fn build(self) -> Result<()> {
        // Obtain API discovery documents 
        let items = discover_specific_apis(&self.client, self.services.clone()).await?;
        let mut docs: Vec<DiscoveryDocument> = Vec::new();
        for item in items {
            let doc = item.get_discovery_document(&self.client).await?;
            docs.push(doc);
        }

        // print general info
        println!("Number of APIs: {}", docs.len());
        println!("Looking into first api: {}", docs[0].name);
        println!("Root Methods: {:?}\n", docs[0].methods.keys().collect::<Vec<_>>());

        // Print resources and methods    
        let resource_keys = docs[0].resources.keys().collect::<Vec<_>>();
        println!("Resources: {:?}\n", resource_keys);
        for k in resource_keys {
            let methods = docs[0].resources.get(k).unwrap().methods.keys().collect::<Vec<_>>();
            println!("Methods for resource '{}': {:?}", k, methods);
        }

        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_google_service_api_builder() {
        let builder = GoogleServicesAPIBuilder::new()
            .with_service_api("keep")
            .with_service_api("gmail")
            .with_service_api("calendar");

        builder.build().await.unwrap();
    }
}
#[allow(dead_code,unused)]
use super::DiscoveryItem;

const DISCOVERY_API_ENDPOINT: &str = "https://discovery.googleapis.com/discovery/v1/apis";

pub fn get_discovery_api_endpoint() -> String {
    DISCOVERY_API_ENDPOINT.to_string()
}

pub async fn discover_apis(client: &reqwest::Client) -> crate::error::Result<Vec<DiscoveryItem>> {
    let url = get_discovery_api_endpoint();
    let response = client.get(&url).send().await?;
    let json = response.json::<serde_json::Value>().await?;
    
    let items = json.get("items").map(|items| {
        items.as_array().unwrap()
    }).expect("No key \"items\" found in discovery API response.");

    let apis: Vec<DiscoveryItem> = items.iter().map(|item| {
        serde_json::from_value(item.clone()).unwrap()
    }).collect();

    Ok(apis)
}

pub async fn discover_specific_apis(client: &reqwest::Client, apis: Vec<String>) -> crate::error::Result<Vec<DiscoveryItem>> {
    let all_apis = discover_apis(client).await?;
    let specific_apis: Vec<DiscoveryItem> = apis.iter().map(|api| {
        all_apis.iter().find(|item| item.name == *api).unwrap().clone()
    }).collect();

    Ok(specific_apis)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_discovery_api_endpoint() {
        let client = reqwest::Client::new();
        let apis = discover_apis(&client).await.unwrap();
        println!("Number of APIs: {}", apis.len());
    }

    #[tokio::test]
    async fn test_parse_discovery_documents() {
        let client = reqwest::Client::new();
        let apis = discover_apis(&client).await.unwrap();
        let mut success = 0;
        let mut failure = 0;
        for api in apis {
            println!("API: {}", api.name);
            let inner_client = reqwest::Client::new();
            match api.get_discovery_document(&inner_client).await {
                Ok(doc) => {
                    println!("  ==> Title: {}\nDescription: {}", doc.title, doc.description);
                    success += 1;
                },
                Err(e) => {
                    println!("  ==> Error: {}", e);
                    failure += 1;
                }
            }
            if failure > 3 || success > 3 {
                break;
            }
        }
        println!("Success: {}\nFailure: {}", success, failure);
        assert_eq!(failure, 0);
    }

    #[tokio::test]
    async fn test_discover_specific_apis() {
        let client = reqwest::Client::new();
        let apis_i_give_a_fuck_about = vec!["gmail".to_string(), "drive".to_string(), "calendar".to_string()];
        let apis_i_give_a_fuck_about_len = apis_i_give_a_fuck_about.len();
        match discover_specific_apis(&client, apis_i_give_a_fuck_about).await {
            Ok(apis) => {
                let apis_len = apis.len();
                for api in apis {
                    println!("{}", api.name)
                }
                assert_eq!(apis_len, apis_i_give_a_fuck_about_len, "Incorrect number of apis returned");
            },
            Err(e) => {
                panic!("Failed to discover specific apis: {:?}", e);
            }
        }
    }

}
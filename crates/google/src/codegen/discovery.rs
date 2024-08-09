#[allow(dead_code,unused)]
use super::DiscoveryItem;

const DISCOVERY_API_ENDPOINT: &str = "https://discovery.googleapis.com/discovery/v1/apis";

pub fn get_discovery_api_endpoint() -> String {
    DISCOVERY_API_ENDPOINT.to_string()
}

pub async fn discover_apis() -> crate::error::Result<Vec<DiscoveryItem>> {
    let client = reqwest::Client::new();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_discovery_api_endpoint() {
        let apis = discover_apis().await.unwrap();
        println!("Number of APIs: {}", apis.len());
    }

    #[tokio::test]
    async fn test_parse_discovery_documents() {
        let apis = discover_apis().await.unwrap();
        let mut success = 0;
        let mut failure = 0;
        for api in apis {
            println!("API: {}", api.name);
            match api.get_discovery_document().await {
                Ok(doc) => {
                    println!("  ==> Title: {}\nDescription: {}", doc.title, doc.description);
                    success += 1;
                },
                Err(e) => {
                    println!("  ==> Error: {}", e);
                    failure += 1;
                }
            }
            if failure > 50 || success > 50 {
                break;
            }
        }
        println!("Success: {}\nFailure: {}", success, failure);
        assert_eq!(failure, 0);
    }
}
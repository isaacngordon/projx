use google::discover_apis;
use google::Error;


#[tokio::main]
async fn main () -> Result<(), Error> {
    let client = reqwest::Client::new();
    let apis = discover_apis(&client).await?;
    println!("Number of APIs: {}", apis.len());

    let mut success = 0;
    let mut failure = 0;

    for i in 0..apis.len() {
        let api = &apis[i];
        println!("{}/{} API: {}", i+1, apis.len(), api.name);
        match api.get_discovery_document(&client).await {
            Ok(_) => success += 1,
            Err(e) => {
                println!("  ==> Error: {}", e);
                failure += 1;
            }
        }
    }

    println!("Success: {}\nFailure: {}", success, failure);

    Ok(())
}
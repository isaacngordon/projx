use google::discover_apis;
use google::Error;


#[tokio::main]
async fn main () -> Result<(), Error> {
    let apis = discover_apis().await?;
    println!("Number of APIs: {}", apis.len());

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
        // if failure > 50 || success > 50 {
        //     break;
        // }
    }

    Ok(())
}
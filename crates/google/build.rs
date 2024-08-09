
use tokio::runtime::Runtime;
use google::discover_apis;

fn main() {
    println!("----------------------------\nStarting API discovery...\n----------------------------");

    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        match google::discover_apis().await {
            Ok(apis) => println!("Discovered APIs: {:?}", apis),
            Err(e) => eprintln!("Error discovering APIs: {:?}", e),
        }
    });
    println!("----------------------------\nStarting API discovery...\n----------------------------");
}

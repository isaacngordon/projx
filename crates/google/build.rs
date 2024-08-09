
use tokio::runtime::Runtime;

fn main() {
    println!("----------------------------\nStarting API discovery...\n----------------------------");

    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        match crates::google::codegen::discovery::discover_apis().await {
            Ok(apis) => println!("Discovered APIs: {:?}", apis),
            Err(e) => eprintln!("Error discovering APIs: {:?}", e),
        }
    });
    println!("----------------------------\nStarting API discovery...\n----------------------------");
}

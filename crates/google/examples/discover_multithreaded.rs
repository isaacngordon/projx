use google::{discover_apis, Error};
use futures::stream::{self, StreamExt};
use tokio::sync::Mutex; // For safe concurrent access to shared data
use std::sync::Arc; // For shared ownership of the Mutex

#[tokio::main]
async fn main () -> Result<(), Error> {
    let apis = discover_apis().await?;
    println!("Number of APIs: {}", apis.len());

    let start_time = std::time::Instant::now();

    // A shared vector to store successful DiscoveryDocs, wrapped in Arc and Mutex for thread safety
    let discovery_docs = Arc::new(Mutex::new(Vec::new()));
    // Even for the counters we need Arc and Mutex because unlike in previous examples, 
    // there is no definite order in the runtime of the tasks, nor a guarantee that they will not overlap.
    let success_counter = Arc::new(Mutex::new(0));
    let failure_counter = Arc::new(Mutex::new(0));

    // Process APIs concurrently with a limit on the number of concurrent tasks
    let concurrency_limit = 4; // Adjust this as needed
    
    let api_count = apis.len();
    stream::iter(apis)
        .enumerate()
        .map(|(i, api)| {
            // Cloning references to shared data
            let discovery_docs = Arc::clone(&discovery_docs);
            let success_counter = Arc::clone(&success_counter);
            let failure_counter = Arc::clone(&failure_counter);
            async move {
                println!("Starting task {}/{} - API: {}", i + 1, api_count, api.name);
                
                match api.get_discovery_document().await {
                    Ok(doc) => {
                        // Request a lock on the shared data. We do not need to explicitly unlock it,
                        // because the MutexGuard is dropped at the end of the scope.
                        let mut docs = discovery_docs.lock().await;
                        docs.push(doc);

                        // Increment success counter
                        let mut success = success_counter.lock().await;
                        *success += 1;
                        println!("Task {}/{} succeeded. Success count: {}", i + 1, api_count, *success);
                        Ok::<(), Error>(())
                    },
                    Err(e) => {
                        // Increment failure counter
                        let mut failure = failure_counter.lock().await;
                        *failure += 1;
                        println!("Task {}/{} failed. Failure count: {}. Error: {}", i + 1, api_count, *failure, e);
                        Err::<(), Error>(e)
                    }
                }
            }
        })
        .buffer_unordered(concurrency_limit) // Controls the level of concurrency
        .for_each(|_| futures::future::ready(()))
        .await;

    // At this point, all tasks are done
    let discovery_docs = Arc::try_unwrap(discovery_docs)
        .expect("Failed to unwrap Arc")
        .into_inner();

    let success_count = Arc::try_unwrap(success_counter)
        .expect("Failed to unwrap Arc")
        .into_inner();
    
    let failure_count = Arc::try_unwrap(failure_counter)
        .expect("Failed to unwrap Arc")
        .into_inner();

    println!("\nFinal Results:");
    println!("Total successful tasks: {}", success_count);
    println!("Total failed tasks: {}", failure_count);
    println!("Collected {} DiscoveryDocs", discovery_docs.len());

    let elapsed = start_time.elapsed();
    println!("Elapsed time: {}.{:03} seconds", elapsed.as_secs(), elapsed.subsec_millis());

    Ok(())
}

// This demonstrates using futures::stream::FuturesUnordered to process
// task results when they complete rather than waiting for all to complete.
// The StreamExt and Rng traits must be in scope.
use futures::stream::{FuturesUnordered, StreamExt};
use rand::Rng;
use tokio::{
    task,
    time::{sleep, Duration},
};

#[tokio::main]
async fn main() {
    let mut futures = FuturesUnordered::new();
    for i in 0..10 {
        futures.push(task::spawn(async move {
            let ms = rand::thread_rng().gen_range(1000..3000);
            sleep(Duration::from_millis(ms)).await;
            (i, ms)
        }));
    }

    // This may be inefficient if there are a large number of futures
    // because it requires polling each one.
    while let Some(result) = futures.next().await {
        let (task_number, sleep_ms) = result.unwrap();
        println!("task {} slept for {}ms", task_number, sleep_ms);
    }

    println!("done");
}

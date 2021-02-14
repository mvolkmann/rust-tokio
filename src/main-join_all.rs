// This demonstrates using futures::future::join_all to
// wait for all the tasks in a collection to complete
// rather than processing results as each task complete.
// The Rng trait must be in scope.
use futures::future::join_all;
use rand::Rng;
use tokio::{
    task,
    time::{sleep, Duration},
};

#[tokio::main]
async fn main() {
    let mut futures = Vec::new();
    for i in 0..10 {
        futures.push(task::spawn(async move {
            let ms = rand::thread_rng().gen_range(1000..3000);
            sleep(Duration::from_millis(ms)).await;
            (i, ms)
        }));
    }

    let results = join_all(futures).await;
    for result in results {
        let (task_number, sleep_ms) = result.unwrap();
        println!("task {} slept for {}ms", task_number, sleep_ms);
    }

    println!("done");
}

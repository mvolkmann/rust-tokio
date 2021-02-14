// This demonstrates using channels to process task results
// when they complete rather than waiting for all to complete.
// The Rng trait must be in scope.
use rand::Rng;
use std::sync::mpsc;
use tokio::{
    task,
    time::{sleep, Duration},
};

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel();
    for i in 0..10 {
        let tx = mpsc::Sender::clone(&tx);
        // The "move" keyword below is necessary for
        // the closure to take ownership of i and tx.
        task::spawn(async move {
            let ms = rand::thread_rng().gen_range(1000..3000);
            sleep(Duration::from_millis(ms)).await;
            tx.send((i, ms)).unwrap();
        });
    }
    // Need to drop the original sender (tx) so the receiver (rx) so
    // the loop below can exit after the last tx clone goes out of scope.
    drop(tx);

    for (task_number, sleep_ms) in rx {
        println!("task {} slept for {}ms", task_number, sleep_ms);
    }

    println!("done");
}

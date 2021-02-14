// This demonstrates using a stream to process task results
// when they complete rather than waiting for all to complete.
// The Rng trait must be in scope.
use futures::stream::StreamExt;
use rand::Rng;
use tokio::{
    task,
    time::{sleep, Duration},
};

const COUNT: usize = 10;

#[tokio::main]
async fn main() {
    let mut stream = futures::stream::iter(0..COUNT)
        .map(|i| {
            // The "move" keyword below is necessary for
            // the closure to take ownership of i.
            task::spawn(async move {
                let ms = rand::thread_rng().gen_range(1000..3000);
                sleep(Duration::from_millis(ms)).await;
                (i, ms) // sent to stream
            })
        })
        // The next line allows results to be buffered
        // in the order in which the tasks complete rather than
        // in the order in which the tasks were created.
        // This uses FuturesUnordered internally.
        .buffer_unordered(COUNT);

    // While there is another result in the stream ...
    while let Some(result) = stream.next().await {
        //let (task_number, sleep_ms) = result.unwrap();
        match result {
            Ok((task_number, sleep_ms)) => {
                println!("task {} slept for {}ms", task_number, sleep_ms);
            }
            Err(e) => eprintln!("error getting stream result: {:?}", e),
        }
    }

    println!("done");
}

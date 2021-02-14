# rust-tokio

This demonstrates three approaches to processing the results of tokio tasks
in the order in which the tasks complete rather than
the order in which the tasks were created.

The approaches are:

1. Using `futures::stream::FuturesUnordered` to poll a collection of futures
   (see `src/main-FuturesUnordered.rs`)
1. Using channels to send results from tasks to the main thread.
   (see `src/main-channels.rs`)
1. Using a `futures::stream` to stream results from tasks to the main thread.
   (see `src/main-stream.rs`)

It seems likely that the channels approach is the most efficient
because the other two approaches repeatedly poll the futures.

To run each of these, copy the corresponding source file to `src/main.rs`
and enter `cargo run`.

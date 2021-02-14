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

The code in `src/main-join_all.rs` demonstrates
waiting for all the tasks to complete before processing their results,
rather than processing the result of each task as it completes.

A potential issue to consider is the performance impact
of loops that determine when a task result is available.
This is not a significant issue with any of the approaches demonstrated here.
In particular, the `FuturesUnordered` documentation says:
"This structure is optimized to manage a large number of futures.
Futures managed by FuturesUnordered will only be polled
when they generate wake-up notifications. This reduces
the required amount of work needed to poll large numbers of futures."

This is an issue to consider when using `futures::future::join_all`
with a large number of futures.

To run each of these, copy the corresponding source file to `src/main.rs`
and enter `cargo run`.

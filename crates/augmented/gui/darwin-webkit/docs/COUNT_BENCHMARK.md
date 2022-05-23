# Count benchmark

Under `examples/count_benchmark.rs` there's a simple test set-up meant to try to determine the latency of sending
messages through `evaluateJavaScript` and `webkit.messageHandlers.postMessage`.

The test works by:

1. Send a message to JavaScript
2. When a message is received in JavaScript, send a message back
3. When a message is received in Rust, send a message back
4. Measure the time to reach N messages received from Rust in this fashion

This can be illustrated by:

<img src="crates/augmented/gui/darwin-webkit/docs/COUNT_BENCHMARK.png" style="max-width: 300px" width="500" />

**Note:** I did not do several runs, instead I'm only measuring the total time for 100k messages and calculating
an average.

### Results

The results on my 2017 MacBook computer are:

| **Average latency** | **Total messages** | **Total time** |
|---------------------|--------------------|----------------|
| 0.11ms              | 100000             | 11.3s          |

### Notes
* The messages have no content in this test
* When considering having to read content from `WKScriptMessage` onto Rust, latency is considerably worse (~2x).
* At `0.11ms`, we can run ~10 messages in a millisecond / 10k messages a second

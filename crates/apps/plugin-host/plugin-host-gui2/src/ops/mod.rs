#[cfg(test)]
mod test {
    use tokio::sync::broadcast;
    use tokio_stream::wrappers::BroadcastStream;
    use tokio_stream::{Stream, StreamExt};

    #[tokio::test]
    async fn test_streams_usage() {
        type Message = i32;

        let output_stream = {
            let (tx, rx) = broadcast::channel(100);
            let recv_stream = BroadcastStream::new(rx);
            fn process(recv_stream: impl Stream<Item = Message>) -> impl Stream<Item = Message> {
                recv_stream.map(|x| x + 10)
            }

            let output_stream = process(recv_stream.filter_map(|x| x.ok()));
            tx.send(10).unwrap();
            tx.send(20).unwrap();
            tx.send(30).unwrap();
            output_stream
        };
        let result: Vec<i32> = output_stream.collect().await;
        assert_eq!(result, vec![20, 30, 40]);
    }
}

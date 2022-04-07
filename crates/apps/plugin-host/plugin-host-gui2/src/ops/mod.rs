// Augmented Audio: Audio libraries and applications
// Copyright (c) 2022 Pedro Tacla Yamada
//
// The MIT License (MIT)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.
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

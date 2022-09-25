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
pub trait ProducerProcedure {
    type Item;

    fn pull(&mut self) -> Option<Self::Item>;
}

pub struct ProducerActor<T, B: ProducerProcedure<Item = T>> {
    read_block: B,
    producer: ringbuf::Producer<T>,
}

impl<T: std::fmt::Debug, B: ProducerProcedure<Item = T>> ProducerActor<T, B> {
    pub fn new(read_block: B, producer: ringbuf::Producer<T>) -> Self {
        Self {
            read_block,
            producer,
        }
    }

    pub fn pull(&mut self) {
        if let Some(v) = self.read_block.pull() {
            self.producer.push(v).unwrap();
        }
    }
}

pub struct FnProcedure<F>(F);

impl<F> FnProcedure<F> {
    pub fn new(f: F) -> Self {
        Self(f)
    }
}

impl<T, F> ProducerProcedure for FnProcedure<F>
where
    F: Fn() -> Option<T>,
{
    type Item = T;

    fn pull(&mut self) -> Option<Self::Item> {
        (self.0)()
    }
}

pub struct ConsumerActor<T> {
    #[allow(dead_code)]
    rx: ringbuf::Consumer<T>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_send_and_receive() {
        let read_block = FnProcedure::new(|| Some(10));
        let (tx, mut rx) = ringbuf::RingBuffer::new(10).split();
        let mut source = ProducerActor::new(read_block, tx);

        source.pull();
        let value = rx.pop().unwrap();
        assert_eq!(value, 10);
    }
}

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

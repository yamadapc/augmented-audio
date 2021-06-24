pub mod atomic_queue;
mod barrier;
mod bounded_queue;
mod double_checked_locking;
mod lock_free_producer_queue;
mod queue;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

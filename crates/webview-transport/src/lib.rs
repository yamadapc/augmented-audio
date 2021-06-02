#[macro_use]
extern crate async_trait;

pub mod transport;
pub mod websockets;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

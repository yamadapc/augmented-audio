#[macro_use]
extern crate async_trait;

pub mod transport;
pub mod webkit;
pub mod websockets;

pub use transport::WebviewTransport;
pub use websockets::{create_transport_runtime, WebSocketsTransport};

#[cfg(test)]
mod tests {
    #[test]
    fn it_compiles() {
        assert_eq!(4, 4);
    }
}

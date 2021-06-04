#[macro_use]
extern crate async_trait;

pub mod delegating;
pub mod transport;
pub mod webkit;
pub mod websockets;

pub use delegating::DelegatingTransport;
pub use transport::WebviewTransport;
pub use webkit::WebkitTransport;
pub use websockets::{create_transport_runtime, WebSocketsTransport};

#[cfg(test)]
mod tests {
    #[test]
    fn it_compiles() {
        assert_eq!(4, 4);
    }
}

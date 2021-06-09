#[macro_use]
extern crate async_trait;

pub mod delegating;
pub mod transport;
#[cfg(any(target_os = "ios", target_os = "macos"))]
pub mod webkit;
pub mod websockets;

pub use delegating::DelegatingTransport;
pub use transport::WebviewTransport;
#[cfg(any(target_os = "ios", target_os = "macos"))]
pub use webkit::WebkitTransport;
pub use websockets::{create_transport_runtime, WebSocketsTransport};

#[cfg(test)]
mod tests {
    #[test]
    fn it_compiles() {
        assert_eq!(4, 4);
    }
}

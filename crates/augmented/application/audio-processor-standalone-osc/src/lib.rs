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
use std::collections::HashMap;
use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};
use std::time::Duration;

use rosc::{OscMessage, OscPacket};
use thiserror::Error;

pub type OscHandler<C> = Box<dyn Fn(&C, OscMessage) + Send>;

pub struct OscMap<C> {
    map: HashMap<String, OscHandler<C>>,
}

impl<C> Default for OscMap<C> {
    fn default() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}

impl<C> OscMap<C> {
    pub fn add(&mut self, addr: impl Into<String>, handler: OscHandler<C>) {
        self.map.insert(addr.into(), handler);
    }

    pub fn get(&self, addr: &str) -> Option<&OscHandler<C>> {
        self.map.get(addr)
    }
}

#[derive(Error, Debug)]
pub enum OscServerError {
    #[error("IO Error, failed to open socket")]
    IOError(#[from] std::io::Error),
}

pub struct OscServer<C> {
    context: C,
    map: OscMap<C>,
}

impl<C> OscServer<C> {
    pub fn new(context: C, map: OscMap<C>) -> Self {
        Self { context, map }
    }

    pub fn start(&self) -> Result<(), OscServerError> {
        let addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 1449);
        let sock = UdpSocket::bind(addr)?;
        sock.set_read_timeout(Some(Duration::from_millis(500)))?;
        let mut buf = [0u8; rosc::decoder::MTU];

        log::info!("Listening...");
        loop {
            match sock.recv_from(&mut buf) {
                Ok((size, addr)) => {
                    log::info!("Received packet with size {} from: {}", size, addr);
                    if let Ok(packet) = rosc::decoder::decode(&buf[..size]) {
                        self.handle_packet(packet);
                    }
                }
                Err(err) => {
                    log::debug!("Failed to recv from OSC socket {}", err);
                }
            }
        }
    }

    fn handle_packet(&self, packet: OscPacket) {
        match packet {
            OscPacket::Message(msg) => {
                if let Some(handler) = self.map.get(&msg.addr) {
                    handler(&self.context, msg);
                } else {
                    log::debug!("OSC address: {}", msg.addr);
                    log::debug!("OSC arguments: {:?}", msg.args);
                }
            }
            OscPacket::Bundle(bundle) => {
                log::debug!("OSC Bundle: {:?}", bundle);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        wisual_logger::init_from_env();
        let mut map: OscMap<()> = OscMap::default();
        map.add(
            "/volume",
            Box::new(|_, msg| {
                if msg.args.is_empty() {
                    return;
                }
                let value = msg.args[0].clone().float();
                log::info!("Volume changed: {:?} {:?}", value, msg.args);
            }),
        );
        let _server = OscServer::new((), map);
        // server.start();
    }
}

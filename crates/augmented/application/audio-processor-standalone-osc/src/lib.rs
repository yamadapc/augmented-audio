use std::collections::HashMap;
use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use rosc::{OscMessage, OscPacket};
use zeroconf::prelude::*;
use zeroconf::{MdnsService, ServiceRegistration, ServiceType};

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
        self.map.insert(addr.into(), handler.into());
    }

    pub fn get(&self, addr: &str) -> Option<&OscHandler<C>> {
        self.map.get(addr)
    }
}

pub struct OscServer<C> {
    context: C,
    map: OscMap<C>,
}

impl<C> OscServer<C> {
    pub fn new(context: C, map: OscMap<C>) -> Self {
        Self { context, map }
    }

    pub fn build_service(&self) -> impl TMdnsService {
        let mut service = MdnsService::new(ServiceType::new("looper", "udp").unwrap(), 1449);
        let hostname = hostname::get().unwrap();
        let hostname = hostname.to_str().unwrap();
        // service.set_host(hostname);
        // log::info!("Publishing host to {}", hostname);

        service
    }

    pub fn start(&self) {
        let mut service = self.build_service();
        let event_loop = service.register().unwrap();

        let addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 1449);
        let sock = UdpSocket::bind(addr).unwrap();
        sock.set_read_timeout(Some(Duration::from_millis(500)));
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
                Err(_) => {}
            }

            event_loop.poll(Duration::from_secs(0)).unwrap();
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
        let server = OscServer::new((), map);
        server.start();
    }
}

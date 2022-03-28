// = copyright ====================================================================
// Continuous: Live-looper and performance sampler
// Copyright (C) 2022  Pedro Tacla Yamada
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
// = /copyright ===================================================================

import Combine
import Foundation
import Logging
import Network
import OSCKit

protocol OSCSender {
    func send(_ message: OSCMessage) throws
}

/// Wraps OSC and service discovery. Will automatically connect to services published as `_looper._udp` on the local network.
/// If initial service browsing fails (due to lack of permissions for example), tries again every 5s.
///
/// Will connect to OSC clients and keep connections open until they're removed from the network or if there's a failure.
///
/// Broadcasts OSC messages onto each connection.
class OSCClient: NSObject {
    let logger = Logger(label: "com.beijaflor.sequencerui.store.OSCClient")

    var browser = NWBrowser(
        for: .bonjour(type: "_looper._udp", domain: nil),
        using: .udp
    )

    var connections: [NWEndpoint: OSCUdpClient] = [:]
    var services: [NetService: NWEndpoint] = [:]

    override init() {
        super.init()
        start()
    }
}

extension OSCClient: OSCSender {
    func send(_ message: OSCMessage) throws {
        for connectionPair in connections {
            let (endpoint, connection) = connectionPair
            logger.debug("Sending osc message to peer", metadata: [
                "endpoint": .string(endpoint.debugDescription),
            ])
            try connection.send(message)
        }
    }
}

// MARK: Service discovery

extension OSCClient {
    private func start() {
        browser.browseResultsChangedHandler = onBrowseResultsChanged
        browser.stateUpdateHandler = onStateUpdateHandler
        browser.start(queue: DispatchQueue.global(qos: .background))
    }

    private func onStateUpdateHandler(_ state: NWBrowser.State) {
        switch state {
        case .setup:
            break
        case .ready:
            break
        case let .failed(error):
            logger.error("Failed to browse", metadata: [
                "error": .string(error.debugDescription),
            ])
            scheduleRetry()
        case .cancelled:
            break
        case .waiting:
            break
        @unknown default:
            break
        }
    }

    private func scheduleRetry() {
        DispatchQueue.global(qos: .background)
            .schedule(after: .init(.now().advanced(by: DispatchTimeInterval.seconds(3)))) {
                self.browser = NWBrowser(
                    for: .bonjour(type: "_looper._udp", domain: nil),
                    using: .udp
                )
                self.start()
            }
    }

    private func onBrowseResultsChanged(_: Set<NWBrowser.Result>, _ changes: Set<NWBrowser.Result.Change>) {
        logger.debug("Browse results changed")
        for change in changes {
            switch change {
            case let .added(result):
                let endpoint = result.endpoint
                logger.debug("New peer added", metadata: [
                    "endpoint": .string(String(describing: endpoint)),
                    "metadata": .string(result.metadata.debugDescription),
                ])
                addConnection(to: endpoint)
            case let .removed(result):
                let endpoint = result.endpoint
                logger.debug("Peer removed", metadata: [
                    "endpoint": .string(String(describing: endpoint)),
                    "metadata": .string(result.metadata.debugDescription),
                ])
                connections.removeValue(forKey: endpoint)
                continue
            default:
                continue
            }
        }
    }
}

// MARK: Endpoint resolution & connection

extension OSCClient {
    private func addConnection(to endpoint: NWEndpoint) {
        logger.debug("Going to connect to endpoint", metadata: [
            "endpoint": .string(endpoint.debugDescription),
        ])
        if case .service(name: let name, type: let type, domain: let domain, interface: _) = endpoint {
            logger.debug("Starting NetService resolution", metadata: [
                "endpoint": .string(endpoint.debugDescription),
                "name": .string(name),
                "type": .string(type),
                "domain": .string(domain),
            ])

            DispatchQueue.main.async {
                let service = NetService(
                    domain: domain,
                    type: type,
                    name: name
                )
                self.services[service] = endpoint
                service.delegate = self
                service.resolve(withTimeout: 3)
            }
        }
    }

    func connectToService(_ endpoint: NWEndpoint, _ service: NetService) {
        logger.debug("Built NetService", metadata: [
            "hostName": .string(service.hostName ?? "<unknown>"),
            "port": .string(service.port.description),
            "addresses": .string(service.addresses?.description ?? "<unknown>"),
        ])

        if let address = service.addresses?.first {
            do {
                let resolvedAddress = try resolveAddress(address)

                logger.info("Connecting to resolved IP for endpoint", metadata: [
                    "endpoint": .string(endpoint.debugDescription),
                    "resolvedAddress": .string(resolvedAddress),
                ])

                connections[endpoint] = OSCUdpClient(
                    host: resolvedAddress,
                    port: 1449
                )
            } catch {
                logger.error("Failed to resolve address", metadata: [
                    "endpoint": .string(endpoint.debugDescription),
                    "hostName": .string(service.hostName ?? "<unknown>"),
                    "error": .string(error.localizedDescription),
                ])
            }
        }
    }

    /// Resolve address for Data
    func resolveAddress(_ address: Data) throws -> String {
        var hostname = [CChar](repeating: 0, count: Int(NI_MAXHOST))

        try address.withUnsafeBytes { (addressRawBufferPtr: UnsafeRawBufferPointer) in
            let sockAddrPtr: UnsafePointer<sockaddr> = addressRawBufferPtr
                .baseAddress!.assumingMemoryBound(to: sockaddr.self)
            guard getnameinfo(
                sockAddrPtr,
                socklen_t(address.count),
                &hostname,
                socklen_t(hostname.count),
                nil,
                0,
                NI_NUMERICHOST
            ) == 0 else {
                throw NSError(domain: "domain", code: 0, userInfo: ["error": "unable to get ip address"])
            }
        }

        return String(cString: hostname)
    }
}

// MARK: Endpoint resolution - NetServiceDelegate

extension OSCClient: NetServiceDelegate {
    func netServiceDidResolveAddress(_ sender: NetService) {
        defer {
            self.services.removeValue(forKey: sender)
        }

        if let endpoint = services[sender] {
            connectToService(endpoint, sender)
        }
    }

    func netServiceDidStop(_: NetService) {
        logger.warning("Service stopped")
    }

    func netService(_: NetService, didNotResolve errorDict: [String: NSNumber]) {
        logger.error("Failed to resolve address", metadata: [
            "errorDict": .string(errorDict.debugDescription),
        ])
    }
}

// MARK: OSC Delegate

extension OSCClient: OSCUdpClientDelegate {
    func client(_: OSCUdpClient, didSendPacket _: OSCPacket, fromHost host: String?, port: UInt16?) {
        logger.debug("OSC Message sent to peer", metadata: [
            "host": .string(host ?? "<unknown>"),
            "port": .string(port.debugDescription),
        ])
    }

    func client(_: OSCUdpClient, didNotSendPacket _: OSCPacket, fromHost host: String?, port: UInt16?, error: Error?) {
        logger.error("Failed to send packet", metadata: [
            "host": .string(host ?? "<unknown>"),
            "port": .string("\(port.debugDescription)"),
            "error": .string(error.debugDescription),
        ])
    }

    func client(_ client: OSCUdpClient, socketDidCloseWithError error: Error) {
        logger.error("Socket closed with error", metadata: [
            "error": .string(error.localizedDescription),
        ])
        if let pair = connections.first(where: { _, value in
            value == client
        }) {
            connections.removeValue(forKey: pair.key)
        }
    }
}

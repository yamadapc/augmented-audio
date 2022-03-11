//
//  File.swift
//  
//
//  Created by Pedro Tacla Yamada on 11/3/2022.
//

import Foundation
import OSCKit
import Logging
import Network
import Combine

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

  var connections: [NWEndpoint:OSCUdpClient] = [:]
  var services: [NetService:NWEndpoint] = [:]

  override init() {
    super.init()
    start()
  }
}

extension OSCClient: OSCSender {
  func send(_ message: OSCMessage) throws {
    for connectionPair in self.connections {
      let (endpoint, connection) = connectionPair
      logger.debug("Sending osc message to peer", metadata: [
        "endpoint": .string(endpoint.debugDescription)
      ])
      try connection.send(message)
    }
  }
}

// MARK: Service discovery
extension OSCClient {
  private func start() {
    browser.browseResultsChangedHandler = self.onBrowseResultsChanged
    browser.stateUpdateHandler = self.onStateUpdateHandler
    browser.start(queue: DispatchQueue.global(qos: .background))
  }

  private func onStateUpdateHandler(_ state: NWBrowser.State) {
    switch state {
    case .setup:
      break
    case .ready:
      break
    case .failed(let error):
      logger.error("Failed to browse", metadata: [
        "error": .string(error.debugDescription)
      ])
      self.scheduleRetry()
    case .cancelled:
      break
    case .waiting(_):
      break
    @unknown default:
      break
    }
  }

  private func scheduleRetry() {
    DispatchQueue.global(qos: .background)
      .schedule(after: .init(.now().advanced(by: DispatchTimeInterval.seconds(3))), {
        self.browser = NWBrowser(
          for: .bonjour(type: "_looper._udp", domain: nil),
          using: .udp
        )
        self.start()
      })
  }

  private func onBrowseResultsChanged(_ results: Set<NWBrowser.Result>, _ changes: Set<NWBrowser.Result.Change>) {
    self.logger.info("Browse results changed")
    for change in changes {
      switch change {
      case .added(let result):
        let endpoint = result.endpoint
        self.logger.info("New peer added", metadata: [
          "endpoint": .string(String(describing: endpoint)),
          "metadata": .string(result.metadata.debugDescription)
        ])
        self.addConnection(to: endpoint)
      case .removed(let result):
        let endpoint = result.endpoint
        self.logger.info("Peer removed", metadata: [
          "endpoint": .string(String(describing: endpoint)),
          "metadata": .string(result.metadata.debugDescription)
        ])
        self.connections.removeValue(forKey: endpoint)
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
    logger.info("Going to connect to endpoint", metadata: [
      "endpoint": .string(endpoint.debugDescription)
    ])
    if case .service(name: let name, type: let type, domain: let domain, interface: _) = endpoint {
      logger.info("Starting NetService resolution", metadata: [
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
    logger.info("Built NetService", metadata: [
      "hostName": .string(service.hostName ?? "<unknown>"),
      "port": .string(service.port.description),
      "addresses": .string(service.addresses?.description ?? "<unknown>"),
    ])

    if let address = service.addresses?.first {
      do {
        let resolvedAddress = try resolveAddress(address)

        logger.info("Resolved IP for endpoint", metadata: [
          "endpoint": .string(endpoint.debugDescription),
          "resolvedAddress": .string(resolvedAddress)
        ])

        self.connections[endpoint] = OSCUdpClient(
          host: resolvedAddress,
          port: 1449
        )
      } catch {
        logger.error("Failed to resolve address", metadata: [
          "endpoint": .string(endpoint.debugDescription),
          "hostName": .string(service.hostName ?? "<unknown>"),
          "error": .string(error.localizedDescription)
        ])
      }
    }
  }

  /// Resolve address for Data
  func resolveAddress(_ address: Data) throws -> String  {
    var hostname = [CChar](repeating: 0, count: Int(NI_MAXHOST))

    try address.withUnsafeBytes { (addressRawBufferPtr: UnsafeRawBufferPointer) -> Void in
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
        throw NSError(domain: "domain", code: 0, userInfo: ["error":"unable to get ip address"])
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

    if let endpoint = self.services[sender] {
      self.connectToService(endpoint, sender)
    }
  }

  func netServiceDidStop(_ sender: NetService) {
    logger.warning("Service stopped")
  }

  func netService(_ sender: NetService, didNotResolve errorDict: [String : NSNumber]) {
    logger.error("Failed to resolve address", metadata: [
      "errorDict": .string(errorDict.debugDescription)
    ])
  }
}

// MARK: OSC Delegate
extension OSCClient: OSCUdpClientDelegate {
  func client(_ client: OSCUdpClient, didSendPacket packet: OSCPacket, fromHost host: String?, port: UInt16?) {
    logger.debug("OSC Message sent to peer", metadata: [
      "host": .string(host ?? "<unknown>"),
      "port": .string(port.debugDescription)
    ])
  }

  func client(_ client: OSCUdpClient, didNotSendPacket packet: OSCPacket, fromHost host: String?, port: UInt16?, error: Error?) {
    logger.error("Failed to send packet", metadata: [
      "host": .string(host ?? "<unknown>"),
      "port": .string("\(port.debugDescription)"),
      "error": .string(error.debugDescription)
    ])
  }

  func client(_ client: OSCUdpClient, socketDidCloseWithError error: Error) {
    logger.error("Socket closed with error", metadata: [
      "error": .string(error.localizedDescription)
    ])
    if let pair = self.connections.first(where: { _key, value in
      value == client
    }) {
      self.connections.removeValue(forKey: pair.key)
    }
  }
}

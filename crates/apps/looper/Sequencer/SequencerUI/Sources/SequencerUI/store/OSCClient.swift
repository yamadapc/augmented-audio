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

func makeOSCClient() -> OSCUdpClient {
  return OSCUdpClient(host: "0.0.0.0", port: 1449)
}

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

  func start() {
    browser.browseResultsChangedHandler = self.onBrowseResultsChanged
    browser.stateUpdateHandler = self.onStateUpdateHandler
    browser.start(queue: DispatchQueue.global(qos: .background))
  }

  func onStateUpdateHandler(_ state: NWBrowser.State) {
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

  func scheduleRetry() {
    DispatchQueue.global(qos: .background)
      .schedule(after: .init(.now().advanced(by: DispatchTimeInterval.seconds(3))), {
        self.browser = NWBrowser(
          for: .bonjour(type: "_looper._udp", domain: nil),
          using: .udp
        )
        self.start()
      })
  }

  func onBrowseResultsChanged(_ results: Set<NWBrowser.Result>, _ changes: Set<NWBrowser.Result.Change>) {
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
      case .removed(_):
        continue
      case .changed(old: let old, new: let new, flags: let flags):
        continue
      default:
        continue
      }
    }
  }

  func send(_ message: OSCMessage) throws {
    for connectionPair in self.connections {
      let (endpoint, connection) = connectionPair
      logger.info("Sending osc message to peer", metadata: [
        "endpoint": .string(endpoint.debugDescription)
      ])
      try connection.send(message)
    }
  }

  func addConnection(to endpoint: NWEndpoint) {
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
        let service = NetService(domain: domain, type: type, name: name)
        service.delegate = self
        service.resolve(withTimeout: 1)
        self.services[service] = endpoint
      }
      return
    }

    let connection = NWConnection(to: endpoint, using: .udp)
    connection.stateUpdateHandler = { state in
      switch state {
      case .ready:
        self.logger.info("Got endpoint", metadata: [
          "innerEndpoint": .string(connection.currentPath?.remoteEndpoint.debugDescription ?? "")
        ])

        if let innerEndpoint = connection.currentPath?.remoteEndpoint,
           case .hostPort(let host, let port) = innerEndpoint
        {
          var url: String? = nil
          switch(host) {
          case .name(_, _):
            break
          case .ipv4(let ip):
            url = ip.debugDescription
            break
          case .ipv6(let ip):
            let ipStr = ip.debugDescription
            let trimIndex = ipStr.firstIndex(of: "%")!.utf16Offset(in: ipStr) - 1
            let trimStrIndex = String.Index(utf16Offset: trimIndex, in: ipStr)
            url = "[\(ipStr[ipStr.startIndex...trimStrIndex])]"
            break
          default:
            break
          }

          if url == nil {
            self.logger.error("Failed to resolve address")
            return
          }

          self.logger.info("Connecting to peer", metadata: [
            "host": .string(url ?? ""),
            "port": .string(port.rawValue.description),
          ])
          let client = OSCUdpClient(
            // interface: //innerEndpoint.interface?.name,
            host: "192.168.1.113",
            port: 1449, // port.rawValue,
            delegate: self
          )
          self.connections[endpoint] = client
        }
      default:
        break
      }
    }
    connection.start(queue: .global(qos: .background))
  }
}

extension OSCClient {
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

extension OSCClient: NetServiceDelegate {
  func netServiceDidResolveAddress(_ sender: NetService) {
    defer {
      self.services.removeValue(forKey: sender)
    }

    logger.info("Resolved address", metadata: [
      "hostName": .string(sender.hostName ?? "<unknown>"),
      "port": .string(sender.port.description),
      "addresses": .string(sender.addresses?.description ?? "<unknown>"),
    ])

    if let address = sender.addresses?.first,
       let endpoint = self.services[sender] {
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
          "hostName": .string(sender.hostName ?? "<unknown>"),
          "error": .string(error.localizedDescription)
        ])
      }
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

extension OSCClient: OSCUdpClientDelegate {
  func client(_ client: OSCUdpClient, didSendPacket packet: OSCPacket, fromHost host: String?, port: UInt16?) {
    logger.info("OSC Message sent to peer", metadata: [
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

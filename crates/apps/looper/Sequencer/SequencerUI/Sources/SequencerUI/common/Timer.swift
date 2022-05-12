import Foundation
import Logging

fileprivate let logger = Logger(label: "com.beijaflor.sequencerui.common.Timer")

/**
 * Run a function and log its duration.
 */
func timeFunction<T>(_ label: Logger.Message, _ block: () -> T) -> T {
  let start = DispatchTime.now()
  let result = block()
  let end = DispatchTime.now()
  let duration = end.uptimeNanoseconds - start.uptimeNanoseconds
  let durationMs = Double(duration) / 1_000_000

  logger.info(label, metadata: [
    "durationMs": .stringConvertible(durationMs),
  ])

  return result
}

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
import Foundation
import Logging

private let logger = Logger(label: "com.beijaflor.sequencerui.common.Timer")

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

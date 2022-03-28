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

/**
 * These are measurements from the audio-thread, pushed on a polling basis.
 *
 * While the audio-thread runs, its processing time per block is measured. We know the time-limit for processing based on
 * sample-rate and block-size and can calculate how much of the budget is used.
 *
 * On a polling basis, a new time measurement is read from a shared memory location. This measurement is collected, aggregated with
 * previous `N` samples and then pushed onto this ObservableObject.
 */
public class ProcessorMetrics: ObservableObject {
    @Published private var inner = Stats(
        averageCpu: 0,
        maximumCpu: 0, averageNanos: 0, maximumNanos: 0
    )

    init() {}

    func getStats() -> Stats {
      return inner
    }

    public func setStats(
        averageCpu: Float,
        maxCpu: Float,
        averageNanos: Float,
        maxNanos: Float
    ) {
        inner = Stats(
            averageCpu: averageCpu,
            maximumCpu: maxCpu,
            averageNanos: averageNanos,
            maximumNanos: maxNanos
        )
    }

    struct Stats {
        let averageCpu: Float
        let maximumCpu: Float
        let averageNanos: Float
        let maximumNanos: Float
    }
}

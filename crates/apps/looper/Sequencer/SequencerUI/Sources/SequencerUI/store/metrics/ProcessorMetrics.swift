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
    @Published var inner = Stats(
        averageCpu: 0,
        maximumCpu: 0, averageNanos: 0, maximumNanos: 0
    )

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

    init() {}

    struct Stats {
        let averageCpu: Float
        let maximumCpu: Float
        let averageNanos: Float
        let maximumNanos: Float
    }
}

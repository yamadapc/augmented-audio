import Combine

/**
 * This is the global playhead position.
 *
 * This is polled from the audio-thread every frame.
 */
public class TimeInfo: ObservableObject {
    @Published public var positionBeats: Double? = nil
    @Published public var tempo: Double? = nil

    init() {}
}


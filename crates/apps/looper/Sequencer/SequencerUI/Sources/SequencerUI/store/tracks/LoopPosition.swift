import Combine

/**
 * This is the position, represented as a number between 0 and 1 within the track playback buffer.
 *
 * This value is pushed by the engine.
 */
public class LoopPosition: ObservableObject {
    @Published public var positionPercent: Float = 0.0

    init() {}
}

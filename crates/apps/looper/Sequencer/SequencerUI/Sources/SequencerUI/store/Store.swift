//
//  File.swift
//
//
//  Created by Pedro Tacla Yamada on 11/3/2022.
//

import Combine
import Foundation
import Logging
import OSCKit

enum TabValue {
    case mix, source, slice, envelope, fx, lfos
}

public enum LooperState {
    case empty, recording, playing, paused, overdubbing, recordingScheduled, playingScheduled
}

extension LooperState {
    var isRecording: Bool { self == .recording || self == .overdubbing }
    var isPlaying: Bool { self == .playing || self == .overdubbing }
    var isEmpty: Bool { self == .empty || self == .recordingScheduled || self == .playingScheduled }
}

public protocol TrackBuffer {
    var count: Int { get }
    subscript(_: Int) -> Float { get }

    func equals(other: TrackBuffer) -> Bool
}

struct UnsafeBufferTrackBuffer {
    let inner: UnsafeBufferPointer<Float32>
}

extension UnsafeBufferTrackBuffer: TrackBuffer {
    var count: Int { inner.count }
    subscript(index: Int) -> Float {
        inner[index]
    }

    func equals(other: TrackBuffer) -> Bool {
        if let otherBuffer = other as? UnsafeBufferTrackBuffer {
            return inner.baseAddress == otherBuffer.inner.baseAddress
        } else {
            return false
        }
    }
}

public class FloatParameter<ParameterId>: ObservableObject, Identifiable {
    public var id: ParameterId
    @Published var label: String
    @Published public var value: Float = 0.0
    var defaultValue: Float
    var range: (Float, Float) = (0.0, 1.0)
    var style: KnobStyle = .normal

    init(id: ParameterId, label: String) {
        self.id = id
        self.label = label
        defaultValue = 0.0
    }

    convenience init(id: ParameterId, label: String, style: KnobStyle, range: (Float, Float), initialValue: Float) {
        self.init(id: id, label: label)
        self.style = style
        self.range = range
        value = initialValue
        defaultValue = initialValue
    }

    convenience init(id: ParameterId, label: String, style: KnobStyle, range: (Float, Float)) {
        self.init(id: id, label: label)
        self.style = style
        self.range = range
    }

    convenience init(id: ParameterId, label: String, initialValue: Float) {
        self.init(id: id, label: label)
        value = initialValue
        defaultValue = initialValue
    }
}

public enum SourceParameterId {
    case start, end, fadeStart, fadeEnd, pitch, speed
}

public typealias SourceParameter = FloatParameter<SourceParameterId>

public class SourceParametersState: ObservableObject {
    var start = SourceParameter(id: .start, label: "Start")
    var end = SourceParameter(id: .end, label: "End", initialValue: 1.0)
    var fadeStart = SourceParameter(id: .fadeStart, label: "Fade start")
    var fadeEnd = SourceParameter(id: .fadeEnd, label: "Fade end")
    var pitch = SourceParameter(id: .pitch, label: "Pitch", style: .center, range: (-1.0, 1.0))
    var speed = SourceParameter(id: .speed, label: "Speed", style: .center, range: (-2.0, 2.0), initialValue: 1.0)

    public var parameters: [SourceParameter] {
        [
            start,
            end,
            fadeStart,
            fadeEnd,
            pitch,
            speed,
        ]
    }

    init() {}
}

public class EnvelopeState: ObservableObject {
    @Published var attack: FloatParameter = .init(id: 0, label: "Attack", initialValue: 0)
    @Published var decay: FloatParameter = .init(id: 2, label: "Decay", initialValue: 0.2)
    @Published var sustain: FloatParameter = .init(id: 3, label: "Sustain", initialValue: 0.8)
    @Published var release: FloatParameter = .init(id: 4, label: "Release", initialValue: 0.3)

    public var parameters: [FloatParameter<Int>] {
        [
            attack,
            decay,
            sustain,
            release,
        ]
    }

    var cancellables: Set<AnyCancellable> = Set()

    init() {
        parameters.forEach { parameter in
            parameter.$value.sink { _ in
                self.objectWillChange.send()
            }.store(in: &cancellables)
        }
    }
}

public class TrackState: ObservableObject {
    @Published var id: Int
    @Published var steps: Set<Int> = Set()
    @Published var buffer: TrackBuffer? = nil
    @Published public var sourceParameters: SourceParametersState = .init()
    @Published public var envelope: EnvelopeState = .init()

    @Published var volume: Float = 1.0

    @Published var lfo1: LFOState = .init()
    @Published var lfo2: LFOState = .init()

    @Published public var looperState: LooperState = .empty

    @Published public var numSamples: UInt = 0
    @Published public var positionPercent: Float = 0.0

    init(id: Int) {
        self.id = id
    }
}

extension TrackState {
    func toggleStep(_ step: Int) {
        objectWillChange.send()
        if steps.contains(step) {
            steps.remove(step)
        } else {
            steps.insert(step)
        }
    }
}

class LFOState: ObservableObject, LFOVisualisationViewModel {
    @Published var frequency: Double = 1
    @Published var amount: Double = 1
}

public protocol SequencerEngine {
    func onClickPlayheadStop()
    func onClickPlayheadPlay()

    func setVolume(track: Int, volume: Float)

    func onClickRecord(track: Int)
    func onClickPlay(track: Int)
    func onClickClear(track: Int)

    func toggleStep(track: Int, step: Int)
}

public class TimeInfo: ObservableObject {
    @Published public var positionSamples: Double = 0.0
    @Published public var positionBeats: Double? = nil
    @Published public var tempo: Double? = nil

    init() {}
}

public class Store: ObservableObject {
    var logger: Logger = .init(label: "com.beijaflor.sequencerui.store.Store")

    @Published var selectedTrack: Int = 1
    @Published var selectedTab: TabValue = .source

    @Published public var trackStates: [TrackState] = (1 ... 8).map { i in
        TrackState(
            id: i
        )
    }

    @Published public var timeInfo: TimeInfo = .init()
    @Published var isPlaying: Bool = false

    public var metronomeVolume: FloatParameter = .init(
        id: 0, label: "Metronome volume", initialValue: 1.0
    )

    var oscClient = OSCClient()

    var engine: SequencerEngine?

    public init(engine: SequencerEngine?) {
        self.engine = engine
    }
}

extension Store {
    func onSelectTab(_ tab: TabValue) {
        selectedTab = tab
    }

    func onClickTrack(_ track: Int) {
        selectedTrack = track
    }

    func onClickStep(_ trackId: Int, _ step: Int) {
        trackStates[trackId - 1].toggleStep(step)
        engine?.toggleStep(track: trackId, step: step)
    }

    func currentTrackState() -> TrackState {
        return trackStates[selectedTrack - 1]
    }
}

public extension Store {
    func setTrackBuffer(trackId: Int, fromAbstractBuffer buffer: TrackBuffer?) {
        trackStates[trackId - 1].buffer = buffer
    }

    func setTrackBuffer(trackId: Int, fromUnsafePointer buffer: UnsafeBufferPointer<Float32>) {
        logger.info("Updating track buffer", metadata: [
            "trackId": .stringConvertible(trackId),
        ])
        trackStates[trackId - 1].buffer = UnsafeBufferTrackBuffer(inner: buffer)
    }
}

protocol RecordingController {
    func onClickRecord()
    func onClickPlay()
    func onClickClear()
}

extension Store {
    func onClickPlayheadStop() {
        engine?.onClickPlayheadStop()
        isPlaying = false
    }

    func onClickPlayheadPlay() {
        engine?.onClickPlayheadPlay()
        isPlaying = true
    }
}

extension Store: RecordingController {
    func onClickRecord() {
        // try? oscClient.send(OSCMessage(
        //     with: "/looper/record"
        // ))

        engine?.onClickRecord(track: selectedTrack)
    }

    func onClickPlay() {
        // try? oscClient.send(OSCMessage(
        //     with: "/looper/play"
        // ))
        engine?.onClickPlay(track: selectedTrack)
    }

    func onClickClear() {
        // try? oscClient.send(OSCMessage(
        //     with: "/looper/clear"
        // ))
        engine?.onClickClear(track: selectedTrack)
    }
}

extension Store {
    func setVolume(track: Int, volume: Float) {
        trackStates[track - 1].volume = volume
        engine?.setVolume(track: track, volume: volume)
    }

    func setParameter(name: String, value: Float) {
        do {
            try oscClient.send(OSCMessage(
                with: "/\(name)",
                arguments: [value]
            ))
        } catch {}
    }
}

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

enum ObjectId: Equatable {
    case
        sourceParameter(trackId: Int, parameterId: SourceParameterId),
        envelopeParameter(trackId: Int, parameterId: EnvelopeParameterId),
        lfoParameter(trackId: Int, lfo: UInt, parameterId: LFOParameterId),
        trackVolume(trackId: Int),
        metronomeVolume
}

class FocusState: ObservableObject {
    @Published var mouseOverObject: ObjectId?

    init() {}
}

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
    var globalId: ObjectId

    @Published var label: String
    @Published public var value: Float = 0.0
    var defaultValue: Float
    var range: (Float, Float) = (0.0, 1.0)
    var style: KnobStyle = .normal

    init(id: ParameterId, globalId: ObjectId, label: String) {
        self.id = id
        self.globalId = globalId
        self.label = label
        defaultValue = 0.0
    }

    convenience init(id: ParameterId, globalId: ObjectId, label: String, style: KnobStyle, range: (Float, Float), initialValue: Float) {
        self.init(id: id, globalId: globalId, label: label)
        self.style = style
        self.range = range
        value = initialValue
        defaultValue = initialValue
    }

    convenience init(id: ParameterId, globalId: ObjectId, label: String, style: KnobStyle, range: (Float, Float)) {
        self.init(id: id, globalId: globalId, label: label)
        self.style = style
        self.range = range
    }

    convenience init(id: ParameterId, globalId: ObjectId, label: String, initialValue: Float) {
        self.init(id: id, globalId: globalId, label: label)
        value = initialValue
        defaultValue = initialValue
    }
}

public enum SourceParameterId {
    case start, end, fadeStart, fadeEnd, pitch, speed
}

public typealias SourceParameter = FloatParameter<SourceParameterId>

public class SourceParametersState: ObservableObject {
    var trackId: Int
    var start: SourceParameter
    var end: SourceParameter
    var fadeStart: SourceParameter
    var fadeEnd: SourceParameter
    var pitch: SourceParameter
    var speed: SourceParameter

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

    init(trackId: Int) {
        self.trackId = trackId
        start = SourceParameter(
            id: .start,
            globalId: .sourceParameter(trackId: trackId, parameterId: .start),
            label: "Start"
        )
        end = SourceParameter(
            id: .end,
            globalId: .sourceParameter(trackId: trackId, parameterId: .end),
            label: "End",
            initialValue: 1.0
        )
        fadeStart = SourceParameter(
            id: .fadeStart,
            globalId: .sourceParameter(trackId: trackId, parameterId: .fadeStart),
            label: "Fade start"
        )
        fadeEnd = SourceParameter(
            id: .fadeEnd,
            globalId: .sourceParameter(trackId: trackId, parameterId: .fadeEnd),
            label: "Fade end"
        )
        pitch = SourceParameter(
            id: .pitch,
            globalId: .sourceParameter(trackId: trackId, parameterId: .pitch),
            label: "Pitch",
            style: .center,
            range: (-1.0, 1.0)
        )
        speed = SourceParameter(
            id: .speed,
            globalId: .sourceParameter(trackId: trackId, parameterId: .speed),
            label: "Speed",
            style: .center,
            range: (-2.0, 2.0),
            initialValue: 1.0
        )
    }
}

public enum EnvelopeParameterId {
    case attack, decay, release, sustain
}

public typealias EnvelopeParameter = FloatParameter<EnvelopeParameterId>

public class EnvelopeState: ObservableObject {
    var trackId: Int
    @Published var attack: EnvelopeParameter
    @Published var decay: EnvelopeParameter
    @Published var sustain: EnvelopeParameter
    @Published var release: EnvelopeParameter

    public var parameters: [EnvelopeParameter] {
        [
            attack,
            decay,
            sustain,
            release,
        ]
    }

    var cancellables: Set<AnyCancellable> = Set()

    init(trackId: Int) {
        self.trackId = trackId
        attack = .init(
            id: .attack,
            globalId: .envelopeParameter(trackId: trackId, parameterId: .attack),
            label: "Attack",
            initialValue: 0
        )
        decay = .init(
            id: .decay,
            globalId: .envelopeParameter(trackId: trackId, parameterId: .decay),
            label: "Decay",
            initialValue: 0.2
        )
        sustain = .init(
            id: .sustain,
            globalId: .envelopeParameter(trackId: trackId, parameterId: .sustain),
            label: "Sustain",
            initialValue: 0.8
        )
        release = .init(
            id: .release,
            globalId: .envelopeParameter(trackId: trackId, parameterId: .release),
            label: "Release",
            initialValue: 0.3
        )

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
    @Published public var sourceParameters: SourceParametersState
    @Published public var envelope: EnvelopeState

    @Published var volumeParameter: FloatParameter<Int>

    @Published var lfo1: LFOState
    @Published var lfo2: LFOState

    @Published public var looperState: LooperState = .empty

    @Published public var numSamples: UInt = 0
    @Published public var positionPercent: Float = 0.0

    init(id: Int) {
        self.id = id
        volumeParameter = .init(
            id: 0,
            globalId: .trackVolume(trackId: id),
            label: "Volume \(id)",
            initialValue: 1.0
        )
        sourceParameters = .init(trackId: id)
        envelope = .init(trackId: id)
        lfo1 = .init(trackId: id, index: 0)
        lfo2 = .init(trackId: id, index: 1)
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

enum LFOParameterId {
    case frequency, amount
}

typealias LFOParameter = FloatParameter<LFOParameterId>

class LFOState: ObservableObject, LFOVisualisationViewModel {
    var trackId: Int
    var index: UInt

    var frequency: Double {
        get {
            Double(frequencyParameter.value)
        }
        set {
            frequencyParameter.value = Float(newValue)
            objectWillChange.send()
        }
    }

    var amount: Double {
        get {
            Double(amountParameter.value)
        }
        set {
            amountParameter.value = Float(newValue)
            objectWillChange.send()
        }
    }

    @Published var frequencyParameter: LFOParameter
    @Published var amountParameter: LFOParameter

    init(trackId: Int, index: UInt) {
        self.trackId = trackId
        self.index = index

        frequencyParameter = .init(
            id: .frequency,
            globalId: .lfoParameter(trackId: trackId, lfo: index, parameterId: .frequency),
            label: "Frequency",
            initialValue: 1.0
        )
        amountParameter = .init(
            id: .amount,
            globalId: .lfoParameter(trackId: trackId, lfo: index, parameterId: .amount),
            label: "Amount",
            initialValue: 1.0
        )
    }
}

public protocol SequencerEngine {
    func onClickPlayheadStop()
    func onClickPlayheadPlay()

    func setVolume(track: Int, volume: Float)
    func setTempo(tempo: Float)

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

    @Published var focusState = FocusState()

    public var metronomeVolume: FloatParameter = .init(
        id: 0,
        globalId: .metronomeVolume,
        label: "Metronome volume",
        initialValue: 1.0
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
        trackStates[track - 1].volumeParameter.value = volume
        engine?.setVolume(track: track, volume: volume)
    }

    func setTempo(tempo: Float) {
        engine?.setTempo(tempo: tempo)
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

//
//  File.swift
//
//
//  Created by Pedro Tacla Yamada on 11/3/2022.
//

import Foundation
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

public class TrackState: ObservableObject {
    @Published var id: Int
    @Published var steps: Set<Int> = Set()
    @Published var buffer: UnsafeBufferPointer<Float32>? = nil

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
    func onClickStep(_ step: Int) {
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

    func onClickRecord(track: Int)
    func onClickPlay(track: Int)
    func onClickClear(track: Int)
}

public class TimeInfo: ObservableObject {
    @Published public var positionSamples: Double = 0.0
    @Published public var positionBeats: Double? = nil
    @Published public var tempo: Double? = nil

    init() {}
}

public class Store: ObservableObject {
    @Published var selectedTrack: Int = 1
    @Published var selectedTab: TabValue = .source

    @Published public var trackStates: [TrackState] = (1 ... 8).map { i in
        TrackState(
            id: i
        )
    }

    @Published public var timeInfo: TimeInfo = .init()
    @Published var isPlaying: Bool = false

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

    func currentTrackState() -> TrackState {
        return trackStates[selectedTrack - 1]
    }
}

public extension Store {
    func setTrackBuffer(trackId: Int, buffer: UnsafeBufferPointer<Float32>) {
        trackStates[trackId].buffer = buffer
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
    func setParameter(name: String, value: Float) {
        do {
            try oscClient.send(OSCMessage(
                with: "/\(name)",
                arguments: [value]
            ))
        } catch {}
    }
}

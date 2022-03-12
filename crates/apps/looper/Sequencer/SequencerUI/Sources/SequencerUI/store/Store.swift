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

class LooperState: ObservableObject {
    @Published var isRecording: Bool = false
    @Published var isPlaying: Bool = false
    @Published var isEmpty: Bool = true

    init() {}
}

class TrackState: ObservableObject {
    @Published var id: Int
    @Published var steps: Set<Int> = Set()
    @Published var looperState: LooperState = .init()

    init(id: Int) {
        self.id = id
    }
}

extension TrackState {
    func onClickRecord() {
        let wasRecording = looperState.isRecording
        looperState.isRecording.toggle()
        if !looperState.isPlaying, wasRecording {
            looperState.isPlaying = true
            looperState.isEmpty = false
        }
    }

    func onClickPlay() {
        looperState.isPlaying.toggle()
    }

    func onClickClear() {
        looperState.isPlaying = false
        looperState.isRecording = false
        looperState.isEmpty = true
    }

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

class Store: ObservableObject {
    @Published var selectedTrack: Int = 1
    @Published var selectedTab: TabValue = .source

    @Published var trackStates: [TrackState] = (1 ... 9).map { i in
        TrackState(
            id: i
        )
    }

    @Published var lfoStates: [LFOState] = (1 ... 9).map { _ in
        LFOState()
    }

    var oscClient = OSCClient()

    init() {}
}

extension Store {
    func onSelectTab(_ tab: TabValue) {
        selectedTab = tab
    }

    func onClickTrack(_ track: Int) {
        selectedTrack = track
    }

    func currentTrackState() -> TrackState {
        return trackStates[selectedTrack]
    }

    func currentLFOState() -> LFOState {
        return lfoStates[selectedTrack]
    }
}

protocol RecordingController {
    func onClickRecord()
    func onClickPlay()
    func onClickClear()
}

extension Store: RecordingController {
    func onClickRecord() {
        try? oscClient.send(OSCMessage(
            with: "/looper/record"
        ))
        currentTrackState().onClickRecord()
    }

    func onClickPlay() {
        try? oscClient.send(OSCMessage(
            with: "/looper/play"
        ))
        currentTrackState().onClickPlay()
    }

    func onClickClear() {
        try? oscClient.send(OSCMessage(
            with: "/looper/clear"
        ))
        currentTrackState().onClickClear()
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

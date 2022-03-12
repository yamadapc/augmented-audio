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
  @Published var looperState: LooperState = LooperState()

  init(id: Int) {
    self.id = id
  }
}

extension TrackState {
  func onClickRecord() {
    let wasRecording = looperState.isRecording
    looperState.isRecording = !looperState.isRecording
    if !looperState.isPlaying && wasRecording {
      looperState.isPlaying = true
    }
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
  @Published var selectedTab: TabValue = .lfos

  @Published var trackStates: [TrackState] = (1...10).map { i in
    TrackState(
      id: i
    )
  }

  @Published var lfoStates: [LFOState] = (1...10).map { i in
    LFOState()
  }

  var oscClient = OSCClient()

  init() {
  }
}

extension Store {
  func onSelectTab(_ tab: TabValue) {
    self.selectedTab = tab
  }

  func onClickTrack(_ track: Int) {
    self.selectedTrack = track
  }

  func currentTrackState() -> TrackState {
    return self.trackStates[self.selectedTrack]
  }

  func currentLFOState() -> LFOState {
    return self.lfoStates[self.selectedTrack]
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
    self.currentTrackState().onClickRecord()
  }

  func onClickPlay() {
    try? oscClient.send(OSCMessage(
      with: "/looper/play"
    ))
  }

  func onClickClear() {
    try? oscClient.send(OSCMessage(
      with: "/looper/clear"
    ))
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

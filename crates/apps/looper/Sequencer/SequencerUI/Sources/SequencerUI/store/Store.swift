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
//
//  File.swift
//
//
//  Created by Pedro Tacla Yamada on 11/3/2022.
//

import Combine
import CoreGraphics
import DequeModule
import Foundation
import Logging
import OSCKit

public class Store: ObservableObject {
    var logger: Logger = .init(label: "com.beijaflor.sequencerui.store.Store")

    public let trackStates: [TrackState] = (1 ... 8).map { i in
        TrackState(
            id: i
        )
    }

    public let timeInfo: TimeInfo = .init()
    public let sceneState = SceneState()
    public let metronomeVolume: FloatParameter = .init(
        id: 0,
        globalId: .metronomeVolume,
        label: "Metronome",
        initialValue: 0.7
    )
    public let processorMetrics = ProcessorMetrics()
    public let midi = MIDIMappingState()

    @Published var selectedTrack: Int = 1
    @Published var selectedTab: TabValue = .source
    @Published var isPlaying: Bool = false
    @Published var midiMappingActive = false

    let focusState = FocusState()
    var oscClient = OSCClient()
    var engine: SequencerEngine?

    public init(engine: SequencerEngine?) {
        self.engine = engine
    }
}

public extension Store {
    func addMidiMessage(message: MIDIMessage) {
        midi.addMidiMessage(message: message)
        if midiMappingActive,
           let object = focusState.selectedObject
        {
            midi.addMapping(id: .cc(message.controllerNumber), objectId: object)
            focusState.selectedObject = nil
            engine?.addMidiMapping(controller: message.controllerNumber, parameterId: object)
        }
    }
}

extension Store {
    func startSequencerStepDrag(_ index: Int, dragMode: DragMode) {
        focusState.draggingSource = .stepId(index)
        focusState.dragMode = dragMode
    }

    func startSceneDrag(_ sceneId: Int) {
        focusState.draggingSource = .sceneId(sceneId)
        focusState.dragMode = .lock
    }

    func endGlobalDrag() {
        if let hoveredId = focusState.mouseOverObject,
           let source = focusState.draggingSource,
           focusState.dragMode == .lock
        {
            startParameterLock(hoveredId, parameterLockProgress: ParameterLockState(
                parameterId: hoveredId,
                source: source
            ))
        } else if let hoveredId = focusState.mouseOverObject,
                  let source = focusState.draggingSource,
                  focusState.dragMode == .copy
        {
            // TODO: - implement copy
        }

        focusState.draggingSource = nil
    }

    func startParameterLock(_ id: ParameterId, parameterLockProgress: ParameterLockState) {
        switch id {
        case .sourceParameter(trackId: let trackId, parameterId: _):
            trackStates[trackId - 1].sourceParameters.parameters
                .first(where: { parameter in parameter.globalId == id })?.parameterLockProgress = parameterLockProgress
        case .envelopeParameter(trackId: let trackId, parameterId: _):
            trackStates[trackId - 1].envelope.parameters
                .first(where: { $0.globalId == id })?.parameterLockProgress = parameterLockProgress
        default:
            return
        }
    }

    func endParameterLock<ParameterId>(_ parameter: FloatParameter<ParameterId>) {
        if let progress = parameter.parameterLockProgress {
            parameter.parameterLockProgress = nil
            parameter.objectWillChange.send()

            let track = currentTrackState()

            switch progress.source {
            case let .stepId(stepId):
                if let existingLock = track.steps[stepId]?.parameterLocks.first(where: { $0.parameterId == progress.parameterId }) {
                    existingLock.newValue = progress.newValue
                } else {
                    track.steps[stepId]?.parameterLocks.append(progress)
                }
                engine?.addParameterLock(
                    track: track.id,
                    step: stepId,
                    parameterId: progress.parameterId,
                    value: progress.newValue!
                )
            case let .sceneId(sceneId):
                let scene = sceneState.scenes[sceneId]
                scene.parameterLocks[progress.parameterId] = progress
                engine?.addSceneParameterLock(
                    sceneId: sceneId,
                    track: track.id,
                    parameterId: progress.parameterId,
                    value: progress.newValue!
                )
            }
            track.objectWillChange.send()
        }
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
        logger.info("Step button clicked", metadata: [
            "trackId": .stringConvertible(trackId),
            "stepId": .stringConvertible(step),
        ])
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

    func setSliceBuffer(trackId: Int, fromAbstractBuffer buffer: SliceBuffer?) {
        trackStates[trackId - 1].sliceBuffer = buffer
        trackStates[trackId - 1].sourceParameters.slice.maximum = (buffer?.count ?? 2) - 1
    }

    func setTrackBuffer(trackId: Int, fromUnsafePointer buffer: UnsafeBufferPointer<Float32>) {
        logger.info("Updating track buffer", metadata: [
            "trackId": .stringConvertible(trackId),
        ])
        trackStates[trackId - 1].buffer = UnsafeBufferTrackBuffer(inner: buffer)
    }
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

extension Store {
    func setVolume(track: Int, volume: Float) {
        trackStates[track - 1].volumeParameter.value = volume
        engine?.setVolume(track: track, volume: volume)
    }

    func setTempo(tempo: Float) {
        engine?.setTempo(tempo: tempo)
    }
}

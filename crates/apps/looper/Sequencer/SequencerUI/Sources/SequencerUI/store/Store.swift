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
import CoreGraphics
import DequeModule
import Foundation
import Logging
import OSCKit

public class Store: ObservableObject {
    var logger: Logger = .init(label: "com.beijaflor.sequencerui.store.Store")

    public let trackStates: [TrackState] = (0 ... 7).map { i in
        TrackState(
            id: i
        )
    }

    public let timeInfo: TimeInfo = .init()
    public let sceneState = SceneState()
    public let metronomeVolume: FloatParameter = .init(
        id: .metronomeVolume,
        label: "Metronome",
        initialValue: 0.7
    )
    public let processorMetrics = ProcessorMetrics()
    public let midi = MIDIMappingState()
    @Published public var isPlaying: Bool = false
    @Published public var selectedTrack: UInt = 0

    @Published var selectedTab: TabValue = .source
    @Published var midiMappingActive = false
    let parameterLockStore = ParameterLockStore()

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
            engine?.addMidiMapping(controller: message.controllerNumber.raw, parameterId: object)
        }
    }
}

extension Store {
    func startDrag(source: ParameterLockSource, dragMode: DragMode) {
        focusState.draggingSource = source
        focusState.dragMode = dragMode
    }

    func startSceneDrag(_ sceneId: SceneId) {
        focusState.draggingSource = .sceneId(sceneId)
        focusState.dragMode = .lock
    }

    func endGlobalDrag() {
        if let hoveredId = focusState.mouseOverObject,
           case let .lfoId(lfoId) = focusState.draggingSource
        {
            currentTrackState().lfos[Int(lfoId.index)].addMapping(parameterId: hoveredId, amount: 1.0)
            parameterLockStore.addLock(
                lock: ParameterLockState(
                    parameterId: hoveredId,
                    source: .lfoId(lfoId)
                )
            )
            engine?.addLFOMapping(
                track: selectedTrack,
                lfo: lfoId.index,
                parameterId: hoveredId,
                value: 1.0
            )
        } else if let hoveredId = focusState.mouseOverObject,
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
            trackStates[Int(trackId)].sourceParameters.parameters
                .first(where: { parameter in parameter.globalId == id })?.parameterLockProgress = parameterLockProgress
        case .envelopeParameter(trackId: let trackId, parameterId: _):
            trackStates[Int(trackId)].envelope.parameters
                .first(where: { $0.globalId == id })?.parameterLockProgress = parameterLockProgress
        case let .lfoParameter(trackId: trackId, lfo: lfo, _):
            if lfo == 0 {
                trackStates[Int(trackId)].lfo1.parameters.first(where: { $0.globalId == id })?.parameterLockProgress = parameterLockProgress
            }
            if lfo == 1 {
                trackStates[Int(trackId)].lfo2.parameters.first(where: { $0.globalId == id })?.parameterLockProgress = parameterLockProgress
            }
        default:
            return
        }
    }

    func endParameterLock(_ parameter: FloatParameter) {
        if let progress = parameter.parameterLockProgress {
            parameter.parameterLockProgress = nil
            parameter.objectWillChange.send()

            let track = currentTrackState()

            switch progress.source {
            case let .stepId(stepId):
                parameterLockStore.addLock(lock: progress)
                engine?.addParameterLock(
                    track: track.id,
                    step: stepId.stepIndex,
                    parameterId: progress.parameterId,
                    value: progress.newValue!
                )
            case let .sceneId(sceneId):
                parameterLockStore.addLock(lock: progress)
                engine?.addSceneParameterLock(
                    sceneId: sceneId.index,
                    track: track.id,
                    parameterId: progress.parameterId,
                    value: progress.newValue!
                )
            default:
                break
            }
            track.objectWillChange.send()
        }
    }
}

extension Store {
    func onSelectTab(_ tab: TabValue) {
        selectedTab = tab
    }

    func onClickTrack(_ track: UInt) {
        selectedTrack = track
    }

    func onClickStep(_ trackId: UInt, _ step: Int) {
        logger.info("Step button clicked", metadata: [
            "trackId": .stringConvertible(trackId),
            "stepId": .stringConvertible(step),
        ])
        trackStates[Int(trackId)].toggleStep(step)
        engine?.toggleStep(track: trackId, step: step)
    }

    func currentTrackState() -> TrackState {
        return trackStates[Int(selectedTrack)]
    }
}

public extension Store {
    func setTrackBuffer(trackId: UInt, fromAbstractBuffer buffer: TrackBuffer?) {
        trackStates[Int(trackId)].buffer = buffer
    }

    func setSliceBuffer(trackId: UInt, fromAbstractBuffer buffer: SliceBuffer?) {
        trackStates[Int(trackId)].sliceBuffer = buffer
        trackStates[Int(trackId)].sourceParameters.slice.maximum = buffer?.count ?? 0
    }

    func setTrackBuffer(trackId: UInt, fromUnsafePointer buffer: UnsafeBufferPointer<Float32>) {
        logger.info("Updating track buffer", metadata: [
            "trackId": .stringConvertible(trackId),
        ])
        trackStates[Int(trackId)].buffer = UnsafeBufferTrackBuffer(inner: buffer)
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

    func removeParameterLock(_ id: ParameterLockId) {
        parameterLockStore.removeLock(id)
        engine?.removeLock(parameterLockId: id)
        if focusState.selectedObject == .parameterLock(source: id.source, parameterId: id.parameterId) {
            focusState.selectedObject = nil
        }
    }
}

extension Store {
    func setTempo(tempo: Float) {
        engine?.setTempo(tempo: tempo)
    }
}

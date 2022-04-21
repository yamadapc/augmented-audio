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
import Foundation
import Logging
import SequencerEngine_private
import SequencerUI

public class EngineController {
    public let store: Store

    private let engine: EngineImpl
    private let logger = Logger(label: "com.beijaflor.sequencer.engine.EngineController")
    private var cancellables: Set<AnyCancellable> = Set()
    private let storeSubscriptionsController: StoreSubscriptionsController

    public init() {
        engine = EngineImpl()
        store = Store(engine: engine)

        storeSubscriptionsController = StoreSubscriptionsController(
            store: store,
            engine: engine
        )

        logger.info("Setting-up store -> engine subscriptions")
        storeSubscriptionsController.setup()
        setupMidiSubscription()
        setupApplicationEventsSubscription()

        logger.info("Setting-up store <- engine polling")
        DispatchQueue.main.async {
            self.flushPollInfo()
            self.flushMetricsInfo()
            self.flushParametersInfo(parameters: allParameters())
        }

        loadInitialState()
    }

    func loadInitialState() {
        for track in store.trackStates {
            let hasBuffer = looper_engine__has_looper_buffer(engine.engine, track.id)
            if hasBuffer {
                readLooperBuffer(track.id)
            }
        }
    }

    func setupMidiSubscription() {
        engine.midi?.sink(receiveValue: { event in
            DispatchQueue.main.async {
                self.store.addMidiMessage(message: MIDIMessage(
                    controllerNumber: MIDIControllerNumber(raw: Int(event.value.controller_number)),
                    value: Int(event.value.value)
                ))
            }
        }).store(in: &cancellables)
    }

    func setupApplicationEventsSubscription() {
        logger.info("Setting-up application events")
        let stream = buildApplicationEventStream(registerStream: { cb in looper_engine__register_events_callback(self.engine.engine, cb) })
        stream.sink(receiveValue: { event in
            switch event.tag {
            case ApplicationEventLooperClipUpdated:
                let looperId = event.application_event_looper_clip_updated.looper_id
                self.logger.info("Looper updated event", metadata: ["looper_id": .stringConvertible(looperId)])
                self.readLooperBuffer(looperId)
            default:
                break
            }
        }).store(in: &cancellables)
    }

    public func loadExampleFileBuffer() {
        DispatchQueue.global(qos: .background).async {
            let exampleBuffer = looper__get_example_buffer()
            let bufferPtr = UnsafeBufferPointer<Float32>(
                start: exampleBuffer.ptr,
                count: Int(exampleBuffer.count)
            )
            DispatchQueue.main.async {
                self.store.setTrackBuffer(trackId: 1, fromUnsafePointer: bufferPtr)
            }
        }
    }

    func flushMetricsInfo() {
        let stats = looper_engine__get_stats(engine.engine)
        store.processorMetrics.setStats(
            averageCpu: stats.average_cpu,
            maxCpu: stats.max_cpu,
            averageNanos: stats.average_nanos,
            maxNanos: stats.max_nanos
        )
        DispatchQueue.main.asyncAfter(deadline: .now().advanced(by: .milliseconds(100))) {
            self.flushMetricsInfo()
        }
    }

    func flushParametersInfo(parameters: [SequencerUI.AnyParameter]) {
        parameters.forEach { parameter in
            let parameterId = parameter.id
            guard let trackId = getTrackId(parameterId),
                  let rustId = getObjectIdRust(parameterId)
            else { return }

            let value = looper_engine__get_parameter_value(self.engine.engine, trackId, rustId)
            switch value.tag {
            case CParameterValueFloat:
                parameter.setFloatValue(value.c_parameter_value_float)
            case CParameterValueInt:
                parameter.setIntValue(value.c_parameter_value_int)
            case CParameterValueEnum:
                parameter.setEnumValue(value.c_parameter_value_enum)
            case CParameterValueBool:
                parameter.setBoolValue(value.c_parameter_value_bool)
            default:
                break
            }
        }

        DispatchQueue.main.asyncAfter(deadline: .now().advanced(by: .milliseconds(16))) {
            self.flushParametersInfo(parameters: parameters)
        }
    }

    func flushPollInfo() {
        let playhead = looper_engine__get_playhead_position(engine.engine)

        for trackState in store.trackStates {
            pollTrackState(trackState)
        }

        // Updating ObservableObject at 60fps causes high CPU usage
        let positionBeats = playhead.position_beats == -1 ? nil : playhead.position_beats
        let tempo = playhead.tempo == -1 ? nil : playhead.tempo
        if abs((store.timeInfo.positionBeats ?? 0.0) - (positionBeats ?? 0.0)) > 0.1 ||
            store.timeInfo.tempo != tempo
        {
            store.timeInfo.positionBeats = positionBeats
            store.timeInfo.tempo = tempo
            store.timeInfo.objectWillChange.send()
        }
        if store.isPlaying != playhead.is_playing {
            store.isPlaying = playhead.is_playing
        }

        // store.allParameters

        DispatchQueue.main.asyncAfter(deadline: .now().advanced(by: .milliseconds(16)), qos: .userInitiated) {
            self.flushPollInfo()
        }
    }

    // This is a super super messy approach, but it is efficient
    fileprivate func pollTrackState(_ trackState: TrackState) {
        let trackId = trackState.id
        pollLooperBuffer(trackId, trackState)
        pollSliceBuffer(trackState, trackId)

        let positionPercent = looper_engine__get_looper_position(engine.engine, trackId)
        if trackState.positionPercent != positionPercent {
            trackState.positionPercent = positionPercent
        }
    }

    fileprivate func pollLooperBuffer(_ trackId: UInt, _ trackState: TrackState) {
        let looperState = convertState(looperState: looper_engine__get_looper_state(engine.engine, trackId))

        if trackState.looperState != looperState {
            if looperState == .playing {
                readLooperBuffer(trackId)
            } else if looperState == .empty {
                store.setTrackBuffer(trackId: trackId, fromAbstractBuffer: nil)
            }
            trackState.looperState = looperState
        }
    }

    fileprivate func pollSliceBuffer(_ trackState: TrackState, _ trackId: UInt) {
        // TODO: - this is a bad strategy; somehow the buffer should be set only on changes
        if trackState.sliceBuffer == nil {
            let sliceBuffer = looper_engine__get_looper_slices(engine.engine, trackId)
            let nativeBuffer = SliceBufferImpl(inner: sliceBuffer!)
            if nativeBuffer.count > 0 {
                store.setSliceBuffer(trackId: trackId, fromAbstractBuffer: nativeBuffer)
                logger.info("Received slice buffer from rust", metadata: [
                    "slice_count": .stringConvertible(nativeBuffer.count),
                ])
            }
        }
    }

    /**
     * Forcefully read the looper buffer from the rust side and update the store
     */
    fileprivate func readLooperBuffer(_ trackId: UInt) {
        let buffer = looper_engine__get_looper_buffer(engine.engine, trackId)
        let trackBuffer = LooperBufferTrackBuffer(inner: buffer!)
        store.setTrackBuffer(trackId: trackId, fromAbstractBuffer: trackBuffer)
    }
}

class LooperBufferTrackBuffer {
    private var inner: OpaquePointer

    init(inner: OpaquePointer) {
        self.inner = inner
    }

    deinit {
        looper_buffer__free(inner)
    }
}

extension LooperBufferTrackBuffer: TrackBuffer {
    var id: Int { inner.hashValue }
    var count: Int { Int(looper_buffer__num_samples(inner)) }
    subscript(index: Int) -> Float {
        looper_buffer__get(inner, UInt(index))
    }

    func equals(other: TrackBuffer) -> Bool {
        if let otherBuffer = other as? LooperBufferTrackBuffer {
            return inner == otherBuffer.inner
        } else {
            return false
        }
    }
}

class SliceBufferImpl {
    private var inner: OpaquePointer

    init(inner: OpaquePointer) {
        self.inner = inner
    }

    deinit {
        slice_buffer__free(inner)
    }
}

extension SliceBufferImpl: SliceBuffer {
    var id: Int { inner.hashValue }
    var count: Int { Int(slice_buffer__length(inner)) }
    subscript(index: Int) -> UInt {
        slice_buffer__get(inner, UInt(index))
    }

    func equals(other: SliceBuffer) -> Bool {
        if let otherBuffer = other as? SliceBufferImpl {
            return inner == otherBuffer.inner
        } else {
            return false
        }
    }
}

func convertState(looperState: SequencerEngine_private.LooperState) -> SequencerUI.LooperState {
    switch looperState {
    case SequencerEngine_private.Recording:
        return SequencerUI.LooperState.recording
    case SequencerEngine_private.Playing:
        return SequencerUI.LooperState.playing
    case SequencerEngine_private.Paused:
        return SequencerUI.LooperState.paused
    case SequencerEngine_private.Overdubbing:
        return SequencerUI.LooperState.overdubbing
    case SequencerEngine_private.RecordingScheduled:
        return SequencerUI.LooperState.recordingScheduled
    case SequencerEngine_private.PlayingScheduled:
        return SequencerUI.LooperState.playingScheduled
    default:
        return SequencerUI.LooperState.empty
    }
}

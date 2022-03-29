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

        logger.info("Setting-up store <- engine polling")
        DispatchQueue.main.async {
            self.flushPollInfo()
            self.flushMetricsInfo()
            self.flushParametersInfo(parameters: allParameters())
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
        var hasChange = false
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

        for (i, trackState) in store.trackStates.enumerated() {
            // trackState.numSamples = looper_engine__get_looper_num_samples(engine.engine, UInt(i))
            let positionPercent = looper_engine__get_looper_position(engine.engine, UInt(i))
            if trackState.positionPercent != positionPercent {
                trackState.positionPercent = positionPercent
            }

            let looperState = convertState(looperState: looper_engine__get_looper_state(engine.engine, UInt(i)))
            if trackState.looperState != looperState {
                trackState.looperState = looperState
                if trackState.looperState == .playing {
                    let buffer = looper_engine__get_looper_buffer(engine.engine, UInt(i))
                    let trackBuffer = LooperBufferTrackBuffer(inner: buffer!)
                    store.setTrackBuffer(trackId: i + 1, fromAbstractBuffer: trackBuffer)
                } else if trackState.looperState == .empty {
                    store.setTrackBuffer(trackId: i + 1, fromAbstractBuffer: nil)
                }
            }

            // TODO: - this is a bad strategy; somehow the buffer should be set only on changes
            if trackState.sliceBuffer == nil {
                let sliceBuffer = looper_engine__get_looper_slices(engine.engine, UInt(i))
                let sliceBufferSize = slice_buffer__length(sliceBuffer)
                if sliceBufferSize > 0 {
                    let nativeBuffer = SliceBufferImpl(inner: sliceBuffer!)
                    store.setSliceBuffer(trackId: i + 1, fromAbstractBuffer: nativeBuffer)
                    logger.info("Received slice buffer from rust", metadata: [
                        "slice_count": .stringConvertible(sliceBufferSize)
                    ])
                }
            }
        }

        // Updating ObservableObject at 60fps causes high CPU usage
        let positionBeats = playhead.position_beats == -1 ? nil : playhead.position_beats
        let tempo = playhead.tempo == -1 ? nil : playhead.tempo
        if abs((store.timeInfo.positionBeats ?? 0.0) - (positionBeats ?? 0.0)) > 0.1 ||
            store.timeInfo.tempo != tempo {
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

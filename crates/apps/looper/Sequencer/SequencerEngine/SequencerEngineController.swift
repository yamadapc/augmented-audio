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
import QuartzCore

func getObjectIdRust(_ id: SequencerUI.ParameterId) -> SequencerEngine_private.ParameterId? {
    switch id {
    case .sourceParameter(trackId: _, parameterId: let parameterId):
        return looper_engine__source_parameter_id(SOURCE_PARAMETER_IDS[parameterId]!)
    case .envelopeParameter(trackId: _, parameterId: let parameterId):
        return looper_engine__envelope_parameter_id(ENVELOPE_PARAMETER_IDS[parameterId]!)
    case .lfoParameter(trackId: _, lfo: let lfo, parameterId: let parameterId):
        return looper_engine__lfo_parameter_id(lfo, LFO_PARAMETER_IDS[parameterId]!)
    default:
        return nil
    }
}

// TODO: - write as hash-map
let SOURCE_PARAMETER_IDS: [SourceParameterId: SequencerEngine_private.SourceParameter] = [
    .start: Start,
    .end: End,
    .fadeStart: FadeStart,
    .fadeEnd: FadeEnd,
    .pitch: Pitch,
    .speed: Speed,
    .loopEnabled: LoopEnabled,
    .sliceEnabled: SliceEnabled,
    .sliceId: SliceId,
]

let LFO_PARAMETER_IDS: [LFOParameterId: SequencerEngine_private.LFOParameter] = [
    LFOParameterId.frequency: Frequency,
    LFOParameterId.amount: Amount,
]

let ENVELOPE_PARAMETER_IDS: [EnvelopeParameterId: SequencerEngine_private.EnvelopeParameter] = [
    EnvelopeParameterId.attack: Attack,
    EnvelopeParameterId.decay: Decay,
    EnvelopeParameterId.release: Release,
    EnvelopeParameterId.sustain: Sustain,
    EnvelopeParameterId.enabled: EnvelopeEnabled,
]

let RUST_QUANTIZE_MODES: [QuantizationMode: CQuantizeMode] = [
    .snapNext: CQuantizeModeSnapNext,
    .snapClosest: CQuantizeModeSnapClosest,
    .none: CQuantizeModeNone,
]

let RUST_TEMPO_CONTROL: [TempoControlMode: SequencerEngine_private.TempoControl] = [
    .setAndFollowGlobalTempo: TempoControlSetGlobalTempo,
    .none: TempoControlNone,
]

public class EngineController {
    public let store: Store

    private let engine: EngineImpl
    private let logger = Logger(label: "com.beijaflor.sequencer.engine.EngineController")
    private var cancellables: Set<AnyCancellable> = Set()

    public init() {
        engine = EngineImpl()
        store = Store(engine: engine)

        logger.info("Setting-up store -> engine subscriptions")
        setupStoreSubscriptions()
        setupMidiSubscription()

        logger.info("Setting-up store <- engine polling")
        DispatchQueue.main.async {
            self.flushPollInfo()
            self.flushMetricsInfo()
        }
    }

    func setupMidiSubscription() {
        engine.midi?.sink(receiveValue: { event in
            DispatchQueue.main.async {
                self.store.addMidiMessage(message: MIDIMessage(
                    controllerNumber: Int(event.value.controller_number),
                    value: Int(event.value.value)
                ))
            }
        }).store(in: &cancellables)
    }

    func setupStoreSubscriptions() {
        store.trackStates.enumerated().forEach { i, trackState in
            trackState.volumeParameter.$value.sink(receiveValue: { volume in
              looper_engine__set_volume(self.engine.engine, UInt(i), volume)
            }).store(in: &cancellables)

            trackState.sourceParameters.parameters.forEach { parameter in
                parameter.$value.sink(receiveValue: { value in
                    let rustParameterId = SOURCE_PARAMETER_IDS[parameter.id]!
                    looper_engine__set_source_parameter(self.engine.engine, UInt(i), rustParameterId, value)
                }).store(in: &cancellables)

                let rustParameterId = SOURCE_PARAMETER_IDS[parameter.id]!
                looper_engine__set_source_parameter(self.engine.engine, UInt(i), rustParameterId, parameter.value)
            }

            trackState.sourceParameters.intParameters.forEach { parameter in
                parameter.$value.sink(receiveValue: { value in
                    let rustParameterId = SOURCE_PARAMETER_IDS[parameter.localId]!
                    looper_engine__set_source_parameter_int(self.engine.engine, UInt(i), rustParameterId, Int32(value))
                }).store(in: &cancellables)
                let rustParameterId = SOURCE_PARAMETER_IDS[parameter.localId]!
                looper_engine__set_source_parameter_int(self.engine.engine, UInt(i), rustParameterId, Int32(parameter.value))
            }

            trackState.sourceParameters.toggles.forEach { toggle in
                toggle.$value.sink(receiveValue: { value in
                    if let rustParameterId = getObjectIdRust(toggle.id) {
                        looper_engine__set_boolean_parameter(
                            self.engine.engine,
                            UInt(i),
                            rustParameterId,
                            value
                        )
                    }
                }).store(in: &cancellables)
                if let rustParameterId = getObjectIdRust(toggle.id) {
                    looper_engine__set_boolean_parameter(
                        self.engine.engine,
                        UInt(i),
                        rustParameterId,
                        toggle.value
                    )
                }
            }

            trackState.lfo2.parameters.forEach { parameter in
              parameter.$value.sink(receiveValue: { value in
                if let rustParameterId = LFO_PARAMETER_IDS[parameter.id] {
                    looper_engine__set_lfo_parameter(
                      self.engine.engine,
                      UInt(i),
                      1,
                      rustParameterId,
                      value
                    )
                  }
              }).store(in: &cancellables)
            }
            trackState.lfo1.parameters.forEach { parameter in
              parameter.$value.sink(receiveValue: { value in
                if let rustParameterId = LFO_PARAMETER_IDS[parameter.id] {
    looper_engine__set_lfo_parameter(
      self.engine.engine, UInt(i), 0, rustParameterId, value)
                  }
              }).store(in: &cancellables)
            }

            trackState.envelope.parameters.forEach { parameter in
                parameter.$value.sink(receiveValue: { value in
                    let rustParameterId = ENVELOPE_PARAMETER_IDS[parameter.id]!
                    looper_engine__set_envelope_parameter(
                        self.engine.engine,
                        UInt(i),
                        rustParameterId,
                        value
                    )
                }).store(in: &cancellables)
                let rustParameterId = ENVELOPE_PARAMETER_IDS[parameter.id]!
                looper_engine__set_envelope_parameter(
                    self.engine.engine,
                    UInt(i),
                    rustParameterId,
                    parameter.value
                )
            }
            trackState.envelope.toggles.forEach { toggle in
                toggle.$value.sink(receiveValue: { value in
                    if let rustParameterId = getObjectIdRust(toggle.id) {
                        looper_engine__set_boolean_parameter(
                            self.engine.engine,
                            UInt(i),
                            rustParameterId,
                            value
                        )
                    }
                }).store(in: &cancellables)
                if let rustParameterId = getObjectIdRust(toggle.id) {
                    looper_engine__set_boolean_parameter(
                        self.engine.engine,
                        UInt(i),
                        rustParameterId,
                        toggle.value
                    )
                }
            }

            trackState.quantizationParameters.quantizationMode.$value.sink(receiveValue: { value in
                looper_engine__set_quantization_mode(self.engine.engine, UInt(i), RUST_QUANTIZE_MODES[value]!)
            }).store(in: &cancellables)

            trackState.quantizationParameters.tempoControlMode.$value.sink(receiveValue: { value in
                looper_engine__set_tempo_control(self.engine.engine, UInt(i), RUST_TEMPO_CONTROL[value]!)
            }).store(in: &cancellables)
        }

        store.sceneState.sceneSlider.$value.sink(receiveValue: { value in
            looper_engine__set_scene_slider_value(self.engine.engine, (value + 1.0) / 2.0)
        }).store(in: &cancellables)

        store.metronomeVolume.$value.sink(receiveValue: { value in
            looper_engine__set_metronome_volume(self.engine.engine, value)
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
                    // TODO: here we should free the previous buffer if it exists
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
                    let nativeBuffer = NativeSliceBuffer(inner: sliceBuffer!)
                    store.setSliceBuffer(trackId: i + 1, fromAbstractBuffer: nativeBuffer)
                    logger.info("Received slice buffer from rust", metadata: [
                        "slice_count": .stringConvertible(sliceBufferSize),
                    ])
                }
            }
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

      DispatchQueue.main.asyncAfter(deadline: .now().advanced(by: .milliseconds(16)), qos: .userInitiated) {
            self.flushPollInfo()
        }
    }
}

struct LooperBufferTrackBuffer {
    var inner: OpaquePointer
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

struct NativeSliceBuffer {
    var inner: OpaquePointer
}

extension NativeSliceBuffer: SliceBuffer {
    var id: Int { inner.hashValue }
    var count: Int { Int(slice_buffer__length(inner)) }
    subscript(index: Int) -> UInt {
        slice_buffer__get(inner, UInt(index))
    }

    func equals(other: SliceBuffer) -> Bool {
        if let otherBuffer = other as? NativeSliceBuffer {
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

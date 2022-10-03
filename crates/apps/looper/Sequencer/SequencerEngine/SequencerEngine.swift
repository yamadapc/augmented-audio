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

/**
 * EngineImpl is a holder for  the *mut LooperEngine* pointer. It creates the audio-engine on init & destroys it when dropped.
 * 
 * It should mostly be a 1-1 mapping of the C API into Swift. Raw FFI calls around the "LooperEngine" type are wrapped by this class.
 */
public class EngineImpl {
    let _effectsService = EffectsServiceImpl()
    let _audioIOSettingsController: AudioIOSettingsControllerImpl

    private var engine: OpaquePointer!
    private let logger = Logger(label: "com.beijaflor.sequencer.engine.EngineImpl")
    var midi: AnyPublisher<MidiEvent, Never>?

    init() {
        logger.info("Initializing rust audio engine", metadata: [
            "bundleIdentifier": .string(Bundle.main.bundleIdentifier ?? "<unknown>"),
            "applicationSupport": .stringConvertible(
                FileManager.default.urls(for: .applicationSupportDirectory, in: .userDomainMask).map {
                    $0.description
                }
            ),
        ])
        engine = looper_engine__new()

        logger.info("Building rust MIDI subscription")
        self._audioIOSettingsController = AudioIOSettingsControllerImpl(engine: engine)
        midi = buildStream(
            registerStream: { callback in
                looper_engine__register_midi_callback(self.engine, callback)
            }
        )
    }

    deinit {
        logger.info("Closing rust audio engine")
        looper_engine__free(engine)
    }
}

extension EngineImpl: SequencerEngine {
    public var effectsService: EffectsService {
        _effectsService
    }
    public var audioIOPreferencesController: AudioIOSettingsController {
        _audioIOSettingsController
    }

    public func getInputLevel() -> Float {
        return looper_engine__get_input_level(engine)
    }

    public func loadFile(atPath path: String) {
        path.withCString { cPath in
            looper_engine__load_file(engine, cPath)
        }
    }

    public func onClickRecord(track: UInt) {
        looper_engine__record(engine, track)
    }

    public func onClickPlay(track: UInt) {
        looper_engine__play(engine, track)
    }

    public func onClickClear(track: UInt) {
        looper_engine__clear(engine, track)
    }

    public func onClickPlayheadStop() {
        looper_engine__playhead_stop(engine)
    }

    public func onClickPlayheadPlay() {
        looper_engine__playhead_play(engine)
    }

    public func setTempo(tempo: Float) {
        looper_engine__set_tempo(engine, tempo)
    }

    public func addParameterLock(track: UInt, step: Int, parameterId: SequencerUI.ParameterId, value: Float) {
        if let rustParameterId = getObjectIdRust(parameterId) {
            looper_engine__add_parameter_lock(
                engine,
                track,
                UInt(step),
                rustParameterId,
                value
            )
        }
    }

    public func addLFOMapping(track: UInt, lfo: UInt, parameterId: SequencerUI.ParameterId, value: Float) {
        if let rustParameterId = getObjectIdRust(parameterId) {
            looper_engine__add_lfo_mapping(engine, track, lfo, rustParameterId, value)
        }
    }

    public func addSceneParameterLock(sceneId: Int, track: UInt, parameterId: SequencerUI.ParameterId, value: Float) {
        if let rustParameterId = getObjectIdRust(parameterId) {
            logger.info("Adding scene parameter lock")
            looper_engine__add_scene_parameter_lock(
                engine,
                UInt(sceneId),
                track,
                rustParameterId,
                value
            )
        }
    }

    public func toggleStep(track: UInt, step: Int) {
        looper_engine__toggle_trigger(engine, track, UInt(step))
    }

    public func removeLock(parameterLockId: ParameterLockId) {
        let logObjectIdMissing = {
            self.logger.warning("Failed to get object ID for parameter", metadata: [
                "parameterId": .string(String(describing: parameterLockId.parameterId))
            ])
        }
        let logTrackIdMissing = {
            self.logger.warning("Failed to get track ID for parameter", metadata: [
                "parameterId": .string(String(describing: parameterLockId.parameterId))
            ])
        }

        switch parameterLockId.source {
        case let .stepId(stepId):
            guard let objectId = getObjectIdRust(parameterLockId.parameterId) else { logObjectIdMissing(); return }
            looper_engine__remove_parameter_lock(
                engine,
                UInt(stepId.trackId),
                UInt(stepId.stepIndex),
                objectId
            )
        case let .sceneId(sceneId):
            guard let trackId = getTrackId(parameterLockId.parameterId) else { logTrackIdMissing(); return }
            guard let objectId = getObjectIdRust(parameterLockId.parameterId) else { logObjectIdMissing(); return }
            looper_engine__remove_scene_parameter_lock(
                engine,
                UInt(sceneId.index),
                trackId,
                objectId
            )
        case let .lfoId(lfoId):
            guard let trackId = getTrackId(parameterLockId.parameterId) else { logTrackIdMissing(); return }
            guard let objectId = getObjectIdRust(parameterLockId.parameterId) else { logObjectIdMissing(); return }
            looper_engine__remove_lfo_mapping(
                engine,
                trackId,
                UInt(lfoId.index),
                objectId
            )
        }
    }

    public func addMidiMapping(controller: Int, parameterId: SequencerUI.ParameterId) {
        if let entityId = getEntityIdRust(parameterId) {
            looper_engine__add_midi_mapping(engine, Int32(controller), entityId)
        } else {
            logger.error("Failed to add MIDI mapping to", metadata: [
                "controller": .string(String(describing: controller)),
                "parameterId": .string(String(describing: parameterId)),
            ])
        }
    }

    public func addEffect(looperId: UInt, effectId: EffectId) {
        looper_engine__add_effect(engine, looperId, effectId)
    }

    public func getLFOSample(mode: SequencerUI.LFOMode, phase: Float) -> Float {
        guard let rustMode = LFO_MODES[mode] else { return 0.0 }
        return looper_engine__get_lfo_sample(rustMode, phase)
    }
}

extension EngineImpl {
    func getParameterValue(looperId: UInt, parameterId: SequencerEngine_private.ParameterId) -> CParameterValue {
        return looper_engine__get_parameter_value(engine, looperId, parameterId)
    }

    func setVolume(_ looperId: UInt, volume: Float) {
        looper_engine__set_volume(engine, looperId, volume)
    }

    func setSourceParameter(_ looperId: UInt, parameterId: SequencerEngine_private.SourceParameter, value: Float) {
        looper_engine__set_source_parameter(engine, looperId, parameterId, value)
    }

    func setSourceParameterInt(_ looperId: UInt, parameterId: SequencerEngine_private.SourceParameter, value: Int32) {
        looper_engine__set_source_parameter_int(engine, looperId, parameterId, value)
    }

    func setBooleanParameter(_ looperId: UInt, parameterId: SequencerEngine_private.ParameterId, value: Bool) {
        looper_engine__set_boolean_parameter(engine, looperId, parameterId, value)
    }

    func setLFOParameter(_ looperId: UInt, parameterId: SequencerEngine_private.LFOParameter, lfoId: UInt, value: Float) {
        looper_engine__set_lfo_parameter(engine, looperId, lfoId, parameterId, value)
    }

    func setSceneSliderValue(value: Float) {
        looper_engine__set_scene_slider_value(engine, value)
    }

    func setMetronomeVolume(volume: Float) {
        looper_engine__set_metronome_volume(engine, volume)
    }

    func setActiveLooper(looperId: UInt) {
        looper_engine__set_active_looper(engine, looperId)
    }

    func setLFOMode(looperId: UInt, lfoId: UInt, value: SequencerEngine_private.LFOMode) {
        looper_engine__set_lfo_mode(engine, looperId, lfoId, value)
    }

    func setEnvelopeParameter(looperId: UInt, parameterId: SequencerEngine_private.EnvelopeParameter, value: Float) {
        looper_engine__set_envelope_parameter(engine, looperId, parameterId, value)
    }

    func setQuantizationMode(looperId: UInt, mode: SequencerEngine_private.CQuantizeMode) {
        looper_engine__set_quantization_mode(engine, looperId, mode)
    }

    func setTempoControl(looperId: UInt, tempoControl: SequencerEngine_private.TempoControl) {
        looper_engine__set_tempo_control(engine, looperId, tempoControl)
    }

    func getPlayheadPosition() -> CTimeInfo {
        return looper_engine__get_playhead_position(engine)
    }

    func getLooperPosition(looperId: UInt) -> Float {
        return looper_engine__get_looper_position(engine, looperId)
    }

    func getLooperState(looperId: UInt) -> SequencerEngine_private.LooperState {
        return looper_engine__get_looper_state(engine, looperId)
    }
}

// MARK: Helpers

extension EngineImpl {
    func setSourceParameter(_ looperId: UInt, parameterId: SequencerUI.SourceParameterId, value: Float) {
        guard let rustId = SOURCE_PARAMETER_IDS[parameterId] else {
            logger.warning("Failed to convert UI parameterID into native repr")
            return
        }

        setSourceParameter(
            looperId,
            parameterId: rustId,
            value: value
        )
    }

    func setSourceParameterInt(_ looperId: UInt, parameterId: SequencerUI.SourceParameterId, value: Int32) {
        guard let rustId = SOURCE_PARAMETER_IDS[parameterId] else {
            logger.warning("Failed to convert UI parameterID into native repr")
            return
        }
        setSourceParameterInt(
            looperId,
            parameterId: rustId,
            value: value
        )
    }

    func setBooleanParameter(_ looperId: UInt, parameterId: SequencerUI.ParameterId, value: Bool) {
        guard let rustParameterId = getObjectIdRust(parameterId) else {
            logger.warning("Failed to convert UI parameterID into native repr")
            return
        }
        setBooleanParameter(looperId, parameterId: rustParameterId, value: value)
    }

    func setLFOParameter(_ looperId: UInt, parameterId: SequencerUI.LFOParameterId, lfoId: UInt, value: Float) {
        let rustParameterId = LFO_PARAMETER_IDS[parameterId]!
        setLFOParameter(looperId, parameterId: rustParameterId, lfoId: lfoId, value: value)
    }

    public func getAnalyticsEnabled() -> Bool? {
        let hasAnalytics = looper_engine__has_analytics_enabled(engine)
        if hasAnalytics {
            return looper_engine__get_analytics_enabled(engine)
        } else {
            return nil
        }
    }

    public func setAnalyticsEnabled(_ value: Bool) {
        looper_engine__set_analytics_enabled(engine, value)
    }
}

// MARK: Metrics

extension EngineImpl {
    func getStats() -> CAudioProcessorMetricsStats {
        return looper_engine__get_stats(engine)
    }
}

// MARK: Buffer

extension EngineImpl {
    func hasLooperBuffer(looperId: UInt) -> Bool {
        return looper_engine__has_looper_buffer(engine, looperId)
    }

    func getLooperBuffer(looperId: UInt) -> LooperBufferTrackBuffer {
        let buffer = looper_engine__get_looper_buffer(engine, looperId)
        return LooperBufferTrackBuffer(inner: buffer!)
    }

    func getLooperSlices(looperId: UInt) -> SliceBufferImpl {
        let sliceBuffer = looper_engine__get_looper_slices(engine, looperId)
        return SliceBufferImpl(inner: sliceBuffer!)
    }
}

// MARK: Events

extension EngineImpl {
    func registerEventsCallback(_ cb: ForeignCallback_ApplicationEvent) {
          looper_engine__register_events_callback(engine, cb)
    }
}

// MARK: Testing
extension EngineImpl {
    public static func getExampleBuffer() -> UnsafeBufferPointer<Float32> {
        let exampleBufferPath = Bundle.main.path(forResource: "example-song.mp3", ofType: nil, inDirectory: nil)
        let exampleBufferCStr = exampleBufferPath!.cString(using: .utf8)!
        return exampleBufferCStr.withUnsafeBufferPointer { ptr in
            let exampleBuffer = looper__get_example_buffer(ptr.baseAddress!)
            let bufferPtr = UnsafeBufferPointer<Float32>(
                start: exampleBuffer.ptr,
                count: Int(exampleBuffer.count)
            )
            return bufferPtr
        }
    }

    public static func initLogging() {
        looper__init_logging()
    }
}

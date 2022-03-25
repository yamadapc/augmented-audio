import Combine
import Foundation
import Logging
import SequencerEngine_private
import SequencerUI

/**
 * This whole framework is really disorganized at the moment.
 *
 * EngineImpl is a holder for  the *mut LooperEngine* pointer. It creates the audio-engine on init & destroys it when dropped.
 */
class EngineImpl {
    var engine: OpaquePointer!
    private let logger = Logger(label: "com.beijaflor.sequencer.engine.EngineImpl")

    var midi: AnyPublisher<MidiEvent, Never>?

    init() {
        logger.info("Initializing rust audio engine")
        engine = looper_engine__new()

        logger.info("Building rust MIDI subscription")
        midi = buildStream(
            registerStream: { callback in
                looper_engine__register_midi_callback(self.engine, callback)
            }
        )
    }

    deinit {
        looper_engine__free(engine)
    }
}

extension EngineImpl: SequencerEngine {
    func setVolume(track: Int, volume: Float) {
        looper_engine__set_volume(engine, UInt(track - 1), volume)
    }

    func onClickRecord(track: Int) {
        looper_engine__record(engine, UInt(track - 1))
    }

    func onClickPlay(track: Int) {
        looper_engine__play(engine, UInt(track - 1))
    }

    func onClickClear(track: Int) {
        looper_engine__clear(engine, UInt(track - 1))
    }

    func onClickPlayheadStop() {
        looper_engine__playhead_stop(engine)
    }

    func onClickPlayheadPlay() {
        looper_engine__playhead_play(engine)
    }

    func setTempo(tempo: Float) {
        looper_engine__set_tempo(engine, tempo)
    }

    func addParameterLock(track: Int, step: Int, parameterId: ObjectId, value: Float) {
        if let rustParameterId = getObjectIdRust(parameterId) {
            looper_engine__add_parameter_lock(
                engine,
                UInt(track - 1),
                UInt(step),
                rustParameterId,
                value
            )
        }
    }

    func addSceneParameterLock(sceneId: Int, track: Int, parameterId: ObjectId, value: Float) {
        if let rustParameterId = getObjectIdRust(parameterId) {
            logger.info("Adding scene parameter lock")
            looper_engine__add_scene_parameter_lock(
                engine,
                UInt(sceneId),
                UInt(track - 1),
                rustParameterId,
                value
            )
        }
    }

    func toggleStep(track: Int, step: Int) {
        looper_engine__toggle_trigger(engine, UInt(track - 1), UInt(step))
        // let voice = looper_engine__get_voice(engine, UInt(step - 1))
    }

    func addMidiMapping(controller _: Int, parameterId _: ObjectId) {}
}

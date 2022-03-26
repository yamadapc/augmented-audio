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

  func addParameterLock(track: Int, step: Int, parameterId: SequencerUI.ParameterId, value: Float) {
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

  func addSceneParameterLock(sceneId: Int, track: Int, parameterId: SequencerUI.ParameterId, value: Float) {
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

  func addMidiMapping(controller _: Int, parameterId _: SequencerUI.ParameterId) {}
}

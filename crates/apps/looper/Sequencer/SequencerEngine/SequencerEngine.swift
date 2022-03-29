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

    func addLFOMapping(track: Int, lfo: UInt, parameterId: SequencerUI.ParameterId, value: Float) {
      if let rustParameterId = getObjectIdRust(parameterId) {
        looper_engine__add_lfo_mapping(engine, UInt(track - 1), lfo, rustParameterId, value)
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

    func removeLock(parameterLockId: ParameterLockId) {
      switch parameterLockId.source {
      case let .stepId(stepId):
          looper_engine__remove_parameter_lock(
            engine,
            UInt(stepId.trackId - 1),
            UInt(stepId.stepIndex),
            getObjectIdRust(parameterLockId.parameterId)!
          )
      case let .sceneId(sceneId):
          looper_engine__remove_scene_parameter_lock(
            engine,
            UInt(sceneId.index),
            getTrackId(parameterLockId.parameterId)!,
            getObjectIdRust(parameterLockId.parameterId)!
          )
      case let .lfoId(lfoId):
          looper_engine__remove_lfo_mapping(
                  engine,
                  getTrackId(parameterLockId.parameterId)!,
                  UInt(lfoId.index),
                  getObjectIdRust(parameterLockId.parameterId)!
          )
      }
    }

    func addMidiMapping(controller: Int, parameterId: SequencerUI.ParameterId) {
        if let entityId = getEntityIdRust(parameterId) {
            looper_engine__add_midi_mapping(engine, Int32(controller), entityId)
        } else {
            logger.error("Failed to add MIDI mapping to", metadata: [
                "controller": .string(String(describing: controller)),
                "parameterId": .string(String(describing: parameterId))
            ])
        }
    }
}

extension EngineImpl {
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
}

// MARK: Helpers
extension EngineImpl {
  func setSourceParameter(_ looperId: UInt, parameterId: SequencerUI.SourceParameterId, value: Float) {
      self.setSourceParameter(
          looperId,
          parameterId: SOURCE_PARAMETER_IDS[parameterId]!,
          value: value
      )
  }

  func setSourceParameterInt(_ looperId: UInt, parameterId: SequencerUI.SourceParameterId, value: Int32) {
      self.setSourceParameterInt(
          looperId,
          parameterId: SOURCE_PARAMETER_IDS[parameterId]!,
          value: value
      )
  }

  func setBooleanParameter(_ looperId: UInt, parameterId: SequencerUI.ParameterId, value: Bool) {
    guard let rustParameterId = getObjectIdRust(parameterId) else {
      logger.warning("Failed to convert UI parameterID into native repr")
      return
    }
    self.setBooleanParameter(looperId, parameterId: rustParameterId, value: value)
  }

  func setLFOParameter(_ looperId: UInt, parameterId: SequencerUI.LFOParameterId, lfoId: UInt, value: Float) {
    let rustParameterId = LFO_PARAMETER_IDS[parameterId]!
    setLFOParameter(looperId, parameterId: rustParameterId, lfoId: lfoId, value: value)
  }
}

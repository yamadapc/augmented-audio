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
import Foundation

/**
 * Protocol to be implemented by audio-engine.
 * These are the methods the UI calls when pushing data/events into the engine.
 */
public protocol SequencerEngine {
    func onClickPlayheadStop()
    func onClickPlayheadPlay()

    func setTempo(tempo: Float)

    func onClickRecord(track: Int)
    func onClickPlay(track: Int)
    func onClickClear(track: Int)

    func toggleStep(track: Int, step: Int)
    func addParameterLock(track: Int, step: Int, parameterId: ParameterId, value: Float)
    func addSceneParameterLock(sceneId: Int, track: Int, parameterId: ParameterId, value: Float)
    func addLFOMapping(track: Int, lfo: UInt, parameterId: SequencerUI.ParameterId, value: Float)
    func addMidiMapping(controller: Int, parameterId: ParameterId)

    func removeLock(parameterLockId: ParameterLockId)
    func loadFile(atPath path: String)
    func addEffect(trackId: Int, effectId: EffectId)
}

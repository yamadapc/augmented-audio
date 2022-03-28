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
import SequencerUI
import SequencerEngine_private

/**
 * Map from swift parameter type into rust parameter type
 */
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

func getTrackId(_ id: SequencerUI.ParameterId) -> UInt? {
    switch id {
    case .sourceParameter(trackId: let trackId, parameterId: _):
        return UInt(trackId - 1)
    case .envelopeParameter(trackId: let trackId, parameterId: _):
        return UInt(trackId - 1)
    case .lfoParameter(trackId: let trackId, lfo: _, parameterId: _):
        return UInt(trackId - 1)
    default:
        return nil
    }
}

let SOURCE_PARAMETER_IDS: [SourceParameterId: SequencerEngine_private.SourceParameter] = [
    .start: Start,
    .end: End,
    .fadeStart: FadeStart,
    .fadeEnd: FadeEnd,
    .pitch: Pitch,
    .speed: Speed,
    .loopEnabled: LoopEnabled,
    .sliceEnabled: SliceEnabled,
    .sliceId: SliceId
]

let LFO_PARAMETER_IDS: [LFOParameterId: SequencerEngine_private.LFOParameter] = [
    LFOParameterId.frequency: Frequency,
    LFOParameterId.amount: Amount
]

let ENVELOPE_PARAMETER_IDS: [EnvelopeParameterId: SequencerEngine_private.EnvelopeParameter] = [
    EnvelopeParameterId.attack: Attack,
    EnvelopeParameterId.decay: Decay,
    EnvelopeParameterId.release: Release,
    EnvelopeParameterId.sustain: Sustain,
    EnvelopeParameterId.enabled: EnvelopeEnabled
]

let RUST_QUANTIZE_MODES: [QuantizationMode: CQuantizeMode] = [
    .snapNext: CQuantizeModeSnapNext,
    .snapClosest: CQuantizeModeSnapClosest,
    .none: CQuantizeModeNone
]

let RUST_TEMPO_CONTROL: [TempoControlMode: SequencerEngine_private.TempoControl] = [
    .setAndFollowGlobalTempo: TempoControlSetGlobalTempo,
    .none: TempoControlNone
]

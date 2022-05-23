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
//
//  File.swift
//
//
//  Created by Pedro Tacla Yamada on 25/3/2022.
//

import Foundation

public indirect enum ParameterId: Equatable, Hashable {
    case
        sourceParameter(trackId: UInt, parameterId: SourceParameterId),

        envelopeParameter(trackId: UInt, parameterId: EnvelopeParameterId),
        lfoParameter(trackId: UInt, lfo: UInt, parameterId: LFOParameterId),
        trackVolume(trackId: UInt),

        parameterLock(source: ParameterLockSource, parameterId: ParameterId),

        recordButton(trackId: UInt),
        playButton(trackId: UInt),
        clearButton(trackId: UInt),
        trackButton(trackId: UInt),
        stepButton(trackId: UInt, stepId: Int),
        effectsParameter(trackId: UInt, slotId: UInt, parameterId: UInt),
        lfo(trackId: UInt, lfoIndex: Int),
        transportPlay,
        transportStop,

        mixPage,
        sourcePage,
        slicePage,
        envelopePage,
        effectsPage,

        quantizationMode(trackId: UInt),
        tempoControl(trackId: UInt),

        sceneSlider,
        sceneButton(sceneId: Int),
        metronomeVolume
}

extension ParameterId {
    func shortHelpString() -> String {
        switch self {
        case let .recordButton(trackId: t):
            return "Track \(t + 1) >  Press once to start recording, press again to stop."
        case let .playButton(trackId: t):
            return "Track \(t + 1) > Start playback"
        case let .clearButton(trackId: t):
            return "Track \(t + 1) > Clear looper contents"
        case .transportPlay:
            return "Play the song"
        case .transportStop:
            return "Stop playback of the song"
        case let .stepButton(trackId: t, stepId: s):
            return "Track \(t + 1) > Step \(s) - Click to toggle this step. The loop will trigger on each active step. Click and drag to parameter lock"
        case let .lfoParameter(trackId: t, lfo: l, parameterId: p):
            return "Track \(t + 1) > LFO \(l + 1) > \(p.shortHelpString()) - Command-click and drag to map the LFO to a parameter"
        case let .envelopeParameter(trackId: t, parameterId: p):
            return "Track \(t + 1) > Envelope > \(p.shortHelpString())"
        case let .sourceParameter(trackId: t, parameterId: p):
            return "Track \(t + 1) > \(p.shortHelpString())"
        case let .trackVolume(trackId: t):
            return "Track \(t + 1) > Volume"
        case let .trackButton(trackId: t):
            return "Track \(t + 1) - Press CMD-\(t + 1) to select this track"
        case let .quantizationMode(trackId: trackId):
            return "Track \(trackId + 1) > Quantization mode - 'Snap next' will quantize by waiting until the end of the bar. 'Snap closest' will snap the recording to the closest bar start/end"
        case let .tempoControl(trackId: trackId):
            return "Track \(trackId + 1) > Tempo control"
        case .sceneSlider:
            return "Scene slider"
        case let .sceneButton(sceneId: sceneId):
            return "Scene \(sceneId) - Click and drag to parameter lock"
        case .metronomeVolume:
            return "Metronome volume"
        case .mixPage:
            return "Track and metronome volumes"
        case .sourcePage:
            return "Source parameters"
        case .slicePage:
            return "Automatic slicing"
        case .envelopePage:
            return "ADSR Envelope"
        case let .lfo(trackId: trackId, lfoIndex: lfoIndex):
            return "Track \(trackId + 1) > LFO \(lfoIndex)"
        case let .parameterLock(source: source, parameterId: parameterId):
            return "Parameter lock >> From \(source.toParameterId()) >> Into \(parameterId)"
        case .effectsPage:
            return "Effects"
        default:
            return "..."
        }
    }
}

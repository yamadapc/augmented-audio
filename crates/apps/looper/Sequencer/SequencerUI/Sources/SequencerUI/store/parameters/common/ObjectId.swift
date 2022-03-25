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

public enum ObjectId: Equatable, Hashable {
    case
        sourceParameter(trackId: Int, parameterId: SourceParameterId),

        envelopeParameter(trackId: Int, parameterId: EnvelopeParameterId),
        lfoParameter(trackId: Int, lfo: UInt, parameterId: LFOParameterId),
        trackVolume(trackId: Int),

        recordButton(trackId: Int?),
        playButton(trackId: Int?),
        clearButton(trackId: Int?),
        trackButton(trackId: Int),
        stepButton(trackId: Int, stepId: Int),
        transportPlay,
        transportStop,

        quantizationMode(trackId: Int),
        tempoControl(trackId: Int),

        sceneSlider,
        sceneButton(sceneId: Int),
        metronomeVolume
}

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

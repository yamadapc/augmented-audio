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
import SwiftUI

public typealias EffectId = UInt

public struct EffectDefinition: Identifiable {
    public let id: EffectId
    public let label: String
    public let parameters: [AnyParameter]
    let color: Color
}

/**
 * The engine will build effect models dynamically
 */
public func buildEffect(
    id: EffectId,
    label: String,
    parameters: [AnyParameter]
) -> EffectDefinition {
    return EffectDefinition(
        id: id,
        label: label,
        parameters: parameters,
        color: Color.red
    )
}

public typealias SlotId = UInt

public class EffectSlotModel: ObservableObject, Identifiable {
    public var id: SlotId
    @Published var definition: EffectDefinition? = nil

    init(slotId: UInt, definition: EffectDefinition?) {
        id = slotId
        self.definition = definition
    }

    convenience init(slotId: UInt) {
        self.init(slotId: slotId, definition: nil)
    }
}

class EffectsRowViewModel: ObservableObject {
    @Published var creatingEffect: SlotId? = nil
    var selectedEffect: SlotId? {
        get { track.selectedSlot }
        set {
            track.selectedSlot = newValue
        }
    }

    let store: Store
    let track: TrackState

    init(store: Store) {
        self.store = store
        track = store.currentTrackState()
        effectDefinitions = store.engine?.effectsService.listEffects() ?? []
    }

    var effectSlots: [EffectSlotModel] {
        track.effectSlots
    }

    var effectDefinitions: [EffectDefinition]

    func addEffect(definition: EffectDefinition?, slotId: UInt) {
        creatingEffect = nil

        var definitionCopy: EffectDefinition?
        if let d = definition {
            definitionCopy = EffectDefinition(
                id: d.id,
                label: d.label,
                parameters: d.parameters.map { parameter in
                    let p = AnyParameter(
                        inner: parameter.inner.copy()
                    )
                    // p.inner.id = .effectsParameter(trackId: track.id, slotId: slotId)
                    return p
                },
                color: d.color
            )
        }
        track.effectSlots[Int(slotId)] = EffectSlotModel(
            slotId: slotId,
            definition: definitionCopy
        )

        let currentTrack = store.selectedTrack
        if let effectId: EffectId = definition?.id {
            store.engine?.addEffect(
                trackId: currentTrack,
                effectId: effectId
            )
        }
    }
}

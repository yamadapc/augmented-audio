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

public enum EffectId {
    case filter, bitcrusher, delay, reverb
}

struct EffectDefinition: Identifiable {
    let id: EffectId
    let label: String
    let color: Color
}

class EffectSlotModel: ObservableObject, Identifiable {
    var id: Int
    @Published var definition: EffectDefinition? = nil

    init(slotId: Int, definition: EffectDefinition?) {
        id = slotId
        self.definition = definition
    }

    convenience init(slotId: Int) {
        self.init(slotId: slotId, definition: nil)
    }
}

class EffectsRowViewModel: ObservableObject {
    @Published var creatingEffect: Int? = nil
    @Published var selectedEffect: Int? = nil

    let store: Store

    init(store: Store) {
        self.store = store
    }

    var effectDefinitions: [EffectDefinition] = [
        .init(id: .filter, label: "Filter", color: SequencerColors.recordColor),
        .init(id: .bitcrusher, label: "Bitcrusher", color: SequencerColors.green),
        .init(id: .bitcrusher, label: "Reverb", color: SequencerColors.purple1),
        .init(id: .delay, label: "Delay", color: SequencerColors.blue),
    ]
    var effectSlots: [EffectSlotModel] = (0 ..< 9).map { i in EffectSlotModel(slotId: i) }

    func addEffect(definition: EffectDefinition?, slotId: Int) {
        creatingEffect = nil
        effectSlots[slotId] = EffectSlotModel(slotId: slotId, definition: definition)
        objectWillChange.send()

        let currentTrack = store.selectedTrack
        if let effectId: EffectId = definition?.id {
            store.engine?.addEffect(trackId: currentTrack, effectId: effectId)
        }
    }
}

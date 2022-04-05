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
import SwiftUI

struct CreateEffectModalView: View {
    var slotId: Int
    @ObservedObject var model: EffectsRowViewModel

    var body: some View {
        ScrollView {
            VStack(alignment: .leading, spacing: 20) {
                ForEach(model.effectDefinitions) { effectDefinition in
                    Button(effectDefinition.label, action: {
                        model.addEffect(definition: effectDefinition, slotId: slotId)
                    })
                    .buttonStyle(.plain)
                    .frame(maxWidth: .infinity, alignment: .leading)
                }.listStyle(.plain)

                Button("Clear", action: {
                    model.addEffect(definition: nil, slotId: slotId)
                })
                .buttonStyle(.plain)
                .frame(maxWidth: .infinity, alignment: .leading)

                Button("Cancel", action: {
                    model.creatingEffect = nil
                })
                .buttonStyle(.plain)
                .frame(maxWidth: .infinity, alignment: .leading)
            }
            .frame(maxWidth: .infinity)
            .padding(PADDING)
            .background(SequencerColors.black0.opacity(0.9))
            .cornerRadius(BORDER_RADIUS)
        }
        .padding(PADDING)
    }
}

struct EffectsRowView: View {
    @ObservedObject var model: EffectsRowViewModel

    init(store: Store) {
        model = EffectsRowViewModel(store: store)
    }

    var body: some View {
        ZStack {
            Rectangle()
                .fill(SequencerColors.white.opacity(0.5))
                .frame(height: 2)
                .frame(maxWidth: .infinity)

            HStack(spacing: 30) {
                ForEach(model.effectSlots) { slot in
                    Rectangle()
                        .fill(
                            slot.definition?.color ?? SequencerColors.white.opacity(0.6)
                        )
                        .frame(width: 40, height: 40)
                        .cornerRadius(BORDER_RADIUS)
                        .gesture(
                            TapGesture(count: 2)
                                .onEnded {
                                    model.creatingEffect = slot.id
                                }
                        )
                        .gesture(
                            TapGesture()
                                .onEnded {
                                    model.selectedEffect = slot.id
                                }
                        )
                }
            }

            if let slotId = model.creatingEffect {
                CreateEffectModalView(slotId: slotId, model: model)
            }
        }
    }
}

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

struct ToggleParameterView: View {
    @ObservedObject var parameter: BooleanParameter
    @EnvironmentObject var store: Store

    var body: some View {
        Button(
            parameter.label,
            action: {
                parameter.value.toggle()
            }
        )
        .buttonStyle(.plain)
        .padding(PADDING)
        .border(
            SequencerColors.blue,
            width: 1.0
        )
        .background(
            SequencerColors.blue.opacity(
                parameter.value ? 0.8 : 0
            )
        )
        .bindToParameterId(store: store, parameterId: parameter.id, showSelectionOverlay: false)
    }
}

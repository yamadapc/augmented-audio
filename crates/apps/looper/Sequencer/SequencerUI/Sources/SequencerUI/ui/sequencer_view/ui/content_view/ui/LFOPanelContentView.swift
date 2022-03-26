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

struct LFOPanelContentView: View {
    @EnvironmentObject var store: Store
    @ObservedObject var lfoState: LFOState

    var body: some View {
        HStack {
            ParameterKnobView(parameter: lfoState.amountParameter)
                .bindToParameter(store: store, parameter: lfoState.amountParameter)

            ParameterKnobView(parameter: lfoState.frequencyParameter

//                label: "LFO frequency",
//                onChanged: { value in
//                    lfoState.frequency = value * (20 - 0.01) + 0.01
//                },
//                formatValue: { value in
//                },
//                value: (lfoState.frequency - 0.01) / (20 - 0.01)
            )
            .bindToParameter(store: store, parameter: lfoState.frequencyParameter)
        }
        .padding(PADDING)
    }
}

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
//  Created by Pedro Tacla Yamada on 19/3/2022.
//

import SwiftUI

struct TopBarView: View {
    @EnvironmentObject var store: Store
    var body: some View {
        VStack(spacing: 0) {
            HStack(spacing: 0) {
                TransportTempoView(timeInfo: store.timeInfo).frame(alignment: .trailing)
                Spacer()
                TransportControlsView()

                Spacer()

                Button(
                    "MIDI",
                    action: {
                        store.midiMappingActive.toggle()
                    }
                )
                .padding(PADDING * 0.5)
                .background(store.midiMappingActive ? SequencerColors.yellow : SequencerColors.black3)
                .buttonStyle(.plain)
                .cornerRadius(BORDER_RADIUS / 2)

                CPUMeterView(processorMetrics: store.processorMetrics)
            }
            .padding(PADDING)

            Rectangle()
                .fill(SequencerColors.white.opacity(0.1))
                .frame(height: 1.0).frame(maxWidth: .infinity)
        }
        .frame(maxWidth: .infinity)
        .background(SequencerColors.black)
    }
}

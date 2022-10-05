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

struct MetronomeToggleButton: View {
    @EnvironmentObject var store: Store
    @ObservedObject var metronomeVolume: FloatParameter

    var body: some View {
        Button(action: {
            if metronomeVolume.value > 0.0 {
                metronomeVolume.value = 0.0
            } else {
                metronomeVolume.value = 0.7
            }
        }) {
            if #available(macOS 11.0, *) {
                Image(systemName: "metronome")
                    .renderingMode(.template)
                    .padding(2)
                    .background(
                        metronomeVolume.value > 0.0
                            ? SequencerColors.yellow.opacity(0.5)
                            : SequencerColors.transparent
                    )
                    .cornerRadius(4)
            } else {
                Text("Metronome")
            }
        }
        .animation(.easeInOut(duration: 0.3), value: metronomeVolume.value)
        .buttonStyle(.plain)
        .frame(maxHeight: 30)
        .bindToParameterId(
            store: store,
            parameterId: .metronomeVolume,
            showSelectionOverlay: false
        )
    }
}

struct TopBarView: View {
    @EnvironmentObject var store: Store
    var body: some View {
        VStack(spacing: 0) {
            HStack(spacing: 0) {
                HStack(spacing: 10) {
                    TransportTempoView(
                        model: TransportTempoViewModel(timeInfo: store.timeInfo)
                    ).frame(alignment: .trailing)

                    MetronomeToggleButton(metronomeVolume: store.metronomeVolume)
                }

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

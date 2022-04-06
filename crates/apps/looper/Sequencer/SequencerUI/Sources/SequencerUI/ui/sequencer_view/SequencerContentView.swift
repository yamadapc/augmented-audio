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

struct SequencerContentView: View {
    @EnvironmentObject var store: Store

    var body: some View {
        HStack(spacing: 0) {
            VStack(alignment: .leading, spacing: 0) {
                Rectangle().fill(Color.white.opacity(0)).frame(height: PADDING)

                VStack(spacing: 0) {
                    VisualisationView()
                        .bindToNilParameter(store: store)

                    TabsRowView(
                        selectedTab: store.selectedTab,
                        onSelectTab: { tab in
                            store.onSelectTab(tab)
                        }
                    )
                    .bindToNilParameter(store: store)

                    SceneSliderView(sceneState: store.sceneState).padding(PADDING)
                        .bindToNilParameter(store: store)

                    HStack(spacing: 0) {
                        TracksPanelContentView()
                            .bindToNilParameter(store: store)
                            .overlay(
                                Rectangle()
                                    .frame(width: 1, height: nil, alignment: .trailing)
                                    .foregroundColor(SequencerColors.black3),
                                alignment: .trailing
                            )
                        LFOStateView()
                    }

                    SequenceView()
                        .bindToNilParameter(store: store)

                    TracksView(
                        selectedTrack: store.selectedTrack,
                        onClickTrack: { i in store.onClickTrack(UInt(i)) }
                    )
                    .bindToNilParameter(store: store)
                }
            }

            if store.midiMappingActive {
                MIDIMappingPanelView(midi: store.midi)
                    .bindToNilParameter(store: store)
            }
        }
    }
}

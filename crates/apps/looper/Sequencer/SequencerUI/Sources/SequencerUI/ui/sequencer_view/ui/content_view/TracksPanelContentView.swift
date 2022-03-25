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

struct TracksPanelContentView: View {
    @EnvironmentObject var store: Store
    @State var isShown: Bool = false

    var body: some View {
        HStack {
            let tracksPanelContentView = HStack(alignment: .top, spacing: 30) {
                switch store.selectedTab {
                case .mix:
                    MixPanelContentView()
                case .source, .slice:
                    SourcePanelContentView(
                        sourceParameters: store.currentTrackState().sourceParameters
                    )

                case .envelope:
                    EnvelopePanelContentView(
                        envelope: store.currentTrackState().envelope
                    )
                case .fx:
                    EffectsPanelContentView()
                case .lfos:
                    HStack(spacing: PADDING) {
                        LFOPanelContentView(lfoState: store.currentTrackState().lfo1)
                        LFOPanelContentView(lfoState: store.currentTrackState().lfo2)
                    }
                }
            }
            .padding(PADDING * 2)
            .frame(maxWidth: .infinity, maxHeight: .infinity)

            tracksPanelContentView
        }
        .frame(maxHeight: .infinity)
        .foregroundColor(SequencerColors.white)
        .background(SequencerColors.black)
    }
}

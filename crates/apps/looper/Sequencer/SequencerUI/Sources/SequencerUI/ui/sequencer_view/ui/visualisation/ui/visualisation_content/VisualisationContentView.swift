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

struct VisualisationContentView: View {
    @EnvironmentObject var store: Store

    var body: some View {
        let currentTrack = store.currentTrackState()
        switch store.selectedTab {
        case .source:
            LoopVisualisationView(trackState: currentTrack)
        case .slice:
            SliceVisualisationView(trackState: currentTrack)
        case .fx:
            EffectsRowView()
        case .envelope:
            EnvelopeVisualisationView(model: currentTrack.envelope)
        case .lfos:
            HStack {
                LFOVisualisationView(model: currentTrack.lfo1)
                LFOVisualisationView(model: currentTrack.lfo2)
            }
        default:
            Text("No tab content").foregroundColor(SequencerColors.white)
        }
    }
}

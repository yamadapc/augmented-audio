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
import OSCKit
import SwiftUI

struct VisualisationView: View {
    @EnvironmentObject var store: Store

    var body: some View {
        HStack {
            RecordingButtonsView(
                store: store,
                trackState: store.currentTrackState()
            )
            ZStack {
                Rectangle()
                    .fill(SequencerColors.black1)
                    .frame(maxWidth: .infinity, maxHeight: .infinity)
                Rectangle()
                    .fill(SequencerColors.black)
                    .cornerRadius(BORDER_RADIUS)
                    .frame(maxWidth: .infinity, maxHeight: .infinity)

                VisualisationContentView()
                    .foregroundColor(SequencerColors.white)
            }
        }
        .padding(EdgeInsets(top: 0, leading: PADDING, bottom: PADDING, trailing: PADDING))
        .frame(maxHeight: 260)
    }
}

struct VisualisationView_Previews: PreviewProvider {
    static var previews: some View {
        Group {
            VisualisationView()
                .environmentObject(Store(engine: nil))
                .previewDisplayName("Normal")

            VisualisationView()
                .environmentObject(Store.recordingScheduled())
                .previewDisplayName("Recording scheduled")

            VisualisationView()
                .environmentObject(Store.recording())
                .previewDisplayName("Recording")

            VisualisationView()
                .environmentObject(Store.playingScheduled())
                .previewDisplayName("Playing scheduled")

            VisualisationView()
                .environmentObject(Store.playing())
                .previewDisplayName("Playing")
        }
    }
}

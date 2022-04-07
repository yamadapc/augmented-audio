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

struct TrackOverlay: View {
    @ObservedObject var position: LoopPosition

    var body: some View {
        ZStack {
            Rectangle()
                .fill(SequencerColors.green.opacity(0.4))
                .frame(width: 70, height: 70)
                .scaleEffect(x: CGFloat(position.positionPercent), y: 1.0, anchor: .leading)
                .cornerRadius(BORDER_RADIUS)
                .allowsHitTesting(false)
        }
    }
}

struct TracksView: View {
    @EnvironmentObject var store: Store
    var selectedTrack: UInt
    var onClickTrack: (Int) -> Void

    var body: some View {
        HStack {
            ForEach(0 ..< 8) { i in
                ContinuousButton(
                    action: {
                        onClickTrack(i)
                    },
                    label: "\(i + 1)",
                    isSelected: selectedTrack == i
                )
                .overlay(
                    TrackOverlay(position: store.trackStates[i].position)
                )
                .bindToParameterId(
                    store: store,
                    parameterId: .trackButton(trackId: UInt(i)),
                    showSelectionOverlay: false
                )
                .testId("trackButton_\(i)")
            }
        }
        .frame(maxWidth: .infinity, alignment: .bottomLeading)
        .padding(PADDING)
    }
}

struct TracksView_Previews: PreviewProvider {
    static var previews: some View {
        TracksView(
            selectedTrack: 1,
            onClickTrack: { _ in }
        )
    }
}

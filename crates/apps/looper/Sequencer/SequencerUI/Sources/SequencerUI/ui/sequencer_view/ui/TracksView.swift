//
//  SwiftUIView.swift
//
//
//  Created by Pedro Tacla Yamada on 11/3/2022.
//

import SwiftUI

struct TrackOverlay: View {
    @ObservedObject var trackState: TrackState

    var body: some View {
        ZStack {
            Rectangle()
                .fill(SequencerColors.green.opacity(0.4))
                .frame(maxWidth: .infinity, maxHeight: .infinity)
                .scaleEffect(x: CGFloat(trackState.positionPercent), y: 1.0, anchor: .leading)
                .cornerRadius(BORDER_RADIUS)
                .allowsHitTesting(false)
            Text("num_samples=\(trackState.numSamples)")
        }
    }
}

struct TracksView: View {
    @EnvironmentObject var store: Store
    var selectedTrack: Int
    var onClickTrack: (Int) -> Void

    var body: some View {
        HStack {
            ForEach(1 ..< 9) { i in
                ZStack {
                    TrackButton(
                        action: {
                            onClickTrack(i)
                        },
                        label: "\(i)",
                        isSelected: selectedTrack == i
                    )
                    TrackOverlay(trackState: store.trackStates[i - 1])
                }
            }

            TrackButton(
                action: {
                    print("")
                },
                label: "Master",
                isSelected: false
            ).frame(maxWidth: .infinity)
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

//
//  SwiftUIView.swift
//
//
//  Created by Pedro Tacla Yamada on 11/3/2022.
//

import SwiftUI

struct TrackOverlay: View {
    @ObservedObject var position: LoopPosition

    var body: some View {
        ZStack {
            Rectangle()
                .fill(SequencerColors.green.opacity(0.4))
                .frame(maxWidth: .infinity, maxHeight: .infinity)
                .scaleEffect(x: CGFloat(position.positionPercent), y: 1.0, anchor: .leading)
                .cornerRadius(BORDER_RADIUS)
                .allowsHitTesting(false)
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
                TrackButton(
                    action: {
                        onClickTrack(i)
                    },
                    label: "\(i)",
                    isSelected: selectedTrack == i
                )
                .overlay(
                    TrackOverlay(position: store.trackStates[i - 1].position)
                )
                .bindToParameterId(
                    store: store,
                    parameterId: .trackButton(trackId: i),
                    showSelectionOverlay: false
                )
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

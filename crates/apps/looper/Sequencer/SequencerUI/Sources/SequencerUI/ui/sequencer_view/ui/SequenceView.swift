//
//  SwiftUIView.swift
//
//
//  Created by Pedro Tacla Yamada on 11/3/2022.
//

import SwiftUI

struct SequenceModel {
    var activeSteps: Set<Int>
}

struct TrackButtonView: View {
    var isBeat: Bool
    var isActive: Bool
    var isPlaying: Bool

    var onClick: () -> Void

    var body: some View {
        Button(
            action: onClick,
            label: {
                Text("")
                    .frame(maxWidth: .infinity, maxHeight: 40, alignment: .center)
                    .contentShape(Rectangle())
                    .foregroundColor(SequencerColors.white)
                    .overlay(
                        RoundedRectangle(cornerRadius: BORDER_RADIUS)
                            .stroke(SequencerColors.black3, lineWidth: 1.0)
                    )
                    .background(
                        (
                            isActive
                                ? SequencerColors.blue
                                : isBeat ? SequencerColors.black1 : SequencerColors.black
                        ).opacity(isPlaying ? 1.0 : 0.8)
                    )
                    .cornerRadius(BORDER_RADIUS)
            }
        )
        .buttonStyle(.plain)
    }
}

extension TrackButtonView: Equatable {
    static func == (lhs: TrackButtonView, rhs: TrackButtonView) -> Bool {
        lhs.isPlaying == rhs.isPlaying && lhs.isBeat == rhs.isBeat && lhs.isActive == rhs.isActive
    }
}

struct ConnectedStepButtonView: View {
    var index: Int
    var store: Store

    @ObservedObject var timeInfo: TimeInfo
    @ObservedObject var track: TrackState

    var body: some View {
        let isActive = track.steps.contains(index)
        let isPlaying = Int((timeInfo.positionBeats ?? -1.0).truncatingRemainder(dividingBy: 4.0) * 4) == index
        let isBeat = index % 4 == 0

        TrackButtonView(
            isBeat: isBeat,
            isActive: isActive,
            isPlaying: isPlaying,
            onClick: { store.onClickStep(track.id, index) }
        ).equatable()
        .bindToParameterId(store: store, parameterId: .stepButton(trackId: track.id, stepId: index))
    }
}

struct SequenceView: View {
    @EnvironmentObject var store: Store

    var body: some View {
        HStack {
            ForEach(0 ..< 16) { i in
                ConnectedStepButtonView(
                    index: i,
                    store: store,
                    timeInfo: store.timeInfo,
                    track: store.currentTrackState()
                )
            }
        }
        .padding(PADDING)
        .background(SequencerColors.black0)
        .frame(maxWidth: .infinity)
    }
}

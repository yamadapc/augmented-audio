//
//  SequencerView.swift
//  Sequencer
//
//  Created by Pedro Tacla Yamada on 28/2/2022.
//

import SwiftUI

let PADDING: Double = 10
let BORDER_RADIUS: Double = 8

struct SequencerView: View {
    @EnvironmentObject var store: Store

    var body: some View {
        VStack(alignment: .leading, spacing: 0) {
            TransportControlsView()
            VisualisationView()
            TabsRowView(
                selectedTab: store.selectedTab,
                onSelectTab: { tab in
                    store.onSelectTab(tab)
                }
            )
            SceneSliderView().padding(PADDING)
            TracksPanelContentView()
            SequenceView()
            TracksView(
                selectedTrack: store.selectedTrack,
                onClickTrack: { i in store.onClickTrack(i) }
            )
        }
        .foregroundColor(SequencerColors.white)
    }
}

struct SequencerView_Previews: PreviewProvider {
    static var previews: some View {
        SequencerView()
    }
}

struct TrackButton: View {
    var action: () -> Void
    var label: String
    var isDisabled: Bool = false
    var isSelected: Bool
    var backgroundColor: Color?

    var body: some View {
        Button(
            action: action,
            label: {
                Text(label)
                    .frame(width: 80.0, height: 80.0, alignment: .center)
                    .contentShape(Rectangle())
                    .foregroundColor(SequencerColors.white)
                    .background(
                        RoundedRectangle(cornerRadius: BORDER_RADIUS)
                            .stroke(
                                isSelected ? SequencerColors.red : SequencerColors.black3,
                                lineWidth: 1.0
                            )
                            .background(self.backgroundColor ?? SequencerColors.black)
                    )
                    .cornerRadius(BORDER_RADIUS)
            }
        )
        .disabled(isDisabled)
        .buttonStyle(.plain)
    }
}

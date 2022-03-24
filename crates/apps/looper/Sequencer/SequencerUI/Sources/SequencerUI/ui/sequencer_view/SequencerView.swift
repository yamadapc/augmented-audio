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
            TopBarView()
                .bindToNilParameter(store: store)

            HStack {
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

                        TracksPanelContentView()
                            .bindToNilParameter(store: store)

                        SequenceView()
                            .bindToNilParameter(store: store)

                        TracksView(
                            selectedTrack: store.selectedTrack,
                            onClickTrack: { i in store.onClickTrack(i) }
                        )
                        .bindToNilParameter(store: store)
                    }
                }
                if store.midiMappingActive {
                    MIDIMappingPanelView(midi: store.midi)
                        .bindToNilParameter(store: store)
                }
            }

            StatusBarView()
                .bindToNilParameter(store: store)
        }
        .bindToNilParameter(store: store)
        .foregroundColor(SequencerColors.white)
        .overlay(buildKeyWatcher(store: store))
        .overlay(GlobalOverlays())
    }
}

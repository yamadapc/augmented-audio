//
//  SequencerView.swift
//  Sequencer
//
//  Created by Pedro Tacla Yamada on 28/2/2022.
//

import SwiftUI

let PADDING: Double = 10
let BORDER_RADIUS: Double = 8

struct MIDIMappingPanelView: View {
    var body: some View {
        VStack(alignment: .leading) {
            VStack {
                Text("MIDI Map")
                    .bold()
                    .padding(PADDING)
                    .frame(maxWidth: .infinity)
                    .background(SequencerColors.black3)
            }
            .frame(maxWidth: .infinity, maxHeight: .infinity, alignment: .top)

            VStack {
                Text("MIDI Monitor")
                    .bold()
                    .padding(PADDING)
                    .frame(maxWidth: .infinity)
                    .background(SequencerColors.black3)
            }
            .frame(maxWidth: .infinity, maxHeight: .infinity, alignment: .top)
        }
        .frame(width: 200, alignment: .topLeading)
        .frame(maxHeight: .infinity, alignment: .topLeading)
        .background(SequencerColors.black)
    }
}

struct SequencerView: View {
    @EnvironmentObject var store: Store

    var body: some View {
        VStack(alignment: .leading, spacing: 0) {
            TopBarView()
                .bindToNilParameter(store: store)

            HStack {
                VStack(alignment: .leading, spacing: 0) {
                    Rectangle().fill(Color.white.opacity(0)).frame(height: PADDING)
                    VisualisationView()
                        .bindToNilParameter(store: store)

                    TabsRowView(
                        selectedTab: store.selectedTab,
                        onSelectTab: { tab in
                            store.onSelectTab(tab)
                        }
                    )
                    .bindToNilParameter(store: store)

                    TracksPanelContentView()
                        .bindToNilParameter(store: store)

                    SceneSliderView(sceneState: store.sceneState).padding(PADDING)
                        .bindToNilParameter(store: store)

                    SequenceView()
                        .bindToNilParameter(store: store)

                    TracksView(
                        selectedTrack: store.selectedTrack,
                        onClickTrack: { i in store.onClickTrack(i) }
                    )
                    .bindToNilParameter(store: store)
                }
                if store.midiMappingActive {
                    MIDIMappingPanelView()
                        .bindToNilParameter(store: store)
                }
            }

            StatusBarView()
                .bindToNilParameter(store: store)
        }
        .bindToNilParameter(store: store)
        .foregroundColor(SequencerColors.white)
        .overlay(buildKeyWatcher())
        .overlay(GlobalOverlays())
    }

    func buildKeyWatcher() -> some View {
        ZStack {
            if #available(macOS 11.0, *) {
                KeyWatcher(
                    onEvent: { key, modifiers in
                        print("key=\(key) modifiers=\(modifiers)")
                    }
                )
            } else {}
        }.allowsHitTesting(false)
    }
}

struct SequencerView_Previews: PreviewProvider {
    static var previews: some View {
        SequencerView()
    }
}

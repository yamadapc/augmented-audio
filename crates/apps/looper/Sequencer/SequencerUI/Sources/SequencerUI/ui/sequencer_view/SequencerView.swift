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

            HStack {
                VStack(alignment: .leading, spacing: 0) {
                    Rectangle().fill(Color.white.opacity(0)).frame(height: PADDING)
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
                if store.midiMappingActive {
                    MIDIMappingPanelView()
                }
            }

            StatusBarView()
        }
        .focusable()
        .foregroundColor(SequencerColors.white)
        .overlay(buildKeyWatcher())
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

//
//  SwiftUIView.swift
//
//
//  Created by Pedro Tacla Yamada on 11/3/2022.
//

import SwiftUI

struct VisualisationContentView: View {
    @EnvironmentObject var store: Store

    var body: some View {
        switch store.selectedTab {
        case .source:
            LoopVisualisationView(trackState: store.currentTrackState())
        case .lfos:
            HStack {
                LFOVisualisationView(model: store.currentLFOState())
                Rectangle()
                    .fill(SequencerColors.black3)
                    .frame(width: 1.0)
                    .frame(maxHeight: .infinity)
                LFOVisualisationView(model: store.currentLFOState())
            }
        default:
            Text("No tab content").foregroundColor(SequencerColors.white)
        }
    }
}

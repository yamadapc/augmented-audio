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
      LoopVisualisationView()
    case .lfos:
      LFOVisualisationView(model: store.currentLFOState())
    default:
      Text("No tab content").foregroundColor(SequencerColors.white)
    }
  }
}

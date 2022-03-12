//
//  SwiftUIView.swift
//  
//
//  Created by Pedro Tacla Yamada on 11/3/2022.
//

import SwiftUI
import OSCKit

struct VisualisationView: View {
  @EnvironmentObject var store: Store

  var body: some View {
    HStack {
      RecordingButtonsView(
        store: store,
        looperState: store.currentTrackState().looperState
      )
      ZStack {
        Rectangle()
          .fill(SequencerColors.black1)
          .frame(maxWidth: .infinity, maxHeight: .infinity)
        Rectangle()
          .fill(SequencerColors.black)
          .cornerRadius(BORDER_RADIUS)
          .frame(maxWidth: .infinity, maxHeight: .infinity)

        VisualisationContentView()
          .foregroundColor(SequencerColors.white)
      }
    }
    .padding(EdgeInsets(top: 0, leading: PADDING, bottom: PADDING, trailing: PADDING))
    .frame(maxHeight: 400)
  }
}

struct VisualisationView_Previews: PreviewProvider {
  static var previews: some View {
    VisualisationView().environmentObject(Store())
  }
}

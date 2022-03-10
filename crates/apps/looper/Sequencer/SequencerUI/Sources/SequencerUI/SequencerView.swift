//
//  SequencerView.swift
//  Sequencer
//
//  Created by Pedro Tacla Yamada on 28/2/2022.
//

import SwiftUI
import OSCKit

let PADDING: Double = 10
let BORDER_RADIUS: Double = 8

func makeOSCClient() -> OSCUdpClient {
  return OSCUdpClient(host: "0.0.0.0", port: 1449)
}

struct SequencerView: View {
    @State
    var selectedTrack: Int = 1
    @State
    var selectedTab: String = "Source"

    var oscClient = makeOSCClient()


    var body: some View {
      VStack(alignment: .leading, spacing: 0) {
        VisualisationView(oscClient: oscClient)
        TabsRowView(
          selectedTab: selectedTab,
          onSelectTab: { tab in
            selectedTab = tab
          }
        )
        SceneSliderView().padding(PADDING)
        TracksPanelContentView(oscClient: oscClient)
        SequenceView()
        TracksView(
            selectedTrack: selectedTrack,
            onClickTrack: { i in
              selectedTrack = i
            }
        )
      }
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
  var isSelected: Bool

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
            .background(SequencerColors.black)
          )
          .cornerRadius(BORDER_RADIUS)
      }
    )
    .buttonStyle(.plain)
  }
}

//
//  SwiftUIView.swift
//  
//
//  Created by Pedro Tacla Yamada on 11/3/2022.
//

import SwiftUI
import OSCKit

struct VisualisationView: View {
  var oscClient: OSCUdpClient

  var body: some View {
    HStack {
      VStack {
        TrackButton(action: {
          try? oscClient.send(OSCMessage(
            with: "/looper/record"
          ))
        }, label: "Record", isSelected: false)
        TrackButton(action: {
          try? oscClient.send(OSCMessage(
            with: "/looper/play"
          ))
        }, label: "Play", isSelected: false)
        TrackButton(action: {
          try? oscClient.send(OSCMessage(
            with: "/looper/clear"
          ))
        }, label: "Clear", isSelected: false)
      }
      ZStack {
        Rectangle()
          .fill(SequencerColors.black1)
          .frame(maxWidth: .infinity, maxHeight: .infinity)
        Rectangle()
          .fill(SequencerColors.black)
          .cornerRadius(BORDER_RADIUS)
          .frame(maxWidth: .infinity, maxHeight: .infinity)
      }
    }
    .padding(PADDING)
    .frame(maxHeight: 400)
  }
}

struct VisualisationView_Previews: PreviewProvider {
  static var previews: some View {
    VisualisationView(oscClient: makeOSCClient())
  }
}

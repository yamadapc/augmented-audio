//
//  SwiftUIView.swift
//  
//
//  Created by Pedro Tacla Yamada on 11/3/2022.
//

import SwiftUI
import OSCKit

struct TracksPanelContentView: View {
  var oscClient: OSCUdpClient

  var body: some View {
    HStack {
      let tracksPanelContentView = HStack(alignment: .top, spacing: 30) {
        HStack(alignment: .center, spacing: 30) {
          KnobView(label: "Normal")
          KnobView(label: "Center").style(.center)
          KnobView(label: "Other")
          KnobView().style(.center)
          KnobView()
          KnobView().style(.center)
          KnobView()

          KnobView(
            onChanged: { value in
              print(value)

              do {
                try oscClient.send(OSCMessage(
                  with: "/volume",
                  arguments: [Float(value)]
                ))
              } catch {}
            }
          )
        }
      }
        .padding(PADDING * 2)
        .frame(maxWidth: .infinity, maxHeight: .infinity)

      tracksPanelContentView
    }.frame(maxHeight: .infinity)
      .foregroundColor(SequencerColors.white)
    .background(SequencerColors.black)    }
}

struct TracksPanelContentView_Previews: PreviewProvider {
  static var previews: some View {
    TracksPanelContentView(oscClient: makeOSCClient())
  }
}

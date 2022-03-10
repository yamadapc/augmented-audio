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
      let tracks = HStack {
        ForEach(1..<11) { i in
          Group {
            TrackButton(
              action: {
                selectedTrack = i
              },
              label: "\(i)",
              isSelected: selectedTrack == i
            )
          }
        }

        TrackButton(
          action: {
            print("")
          },
          label: "Master",
          isSelected: false
        ).frame(maxWidth: .infinity)
      }
      .frame(maxWidth: .infinity, alignment: .bottomLeading)
      .padding(PADDING)

      let tracksPanel = HStack {
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
        .background(SequencerColors.black)

      let tabs = [
        "Mix",
        "Source",
        "Slice",
        "Envelope",
        "FX",
        "LFOs",
      ]
      let tabsRow = HStack {
        ForEach(tabs, id: \.self) { tab in
          let isSelected = tab == selectedTab
          Button(
            action: {
              selectedTab = tab
            },
            label: {
              Text("\(tab)")
                .frame(maxWidth: .infinity, maxHeight: 50, alignment: .center)
                .contentShape(Rectangle())
                .foregroundColor(SequencerColors.white)
                .overlay(
                  RoundedRectangle(cornerRadius: BORDER_RADIUS)
                    .stroke(
                      isSelected ? SequencerColors.red : SequencerColors.black3,
                      lineWidth: 1.0
                    )
                )
                .background(
                  SequencerColors.black
                )
                .cornerRadius(BORDER_RADIUS)
            }
          )
          .buttonStyle(.plain)
        }
      }
      .padding(PADDING)
      .background(SequencerColors.black0)
      .frame(maxWidth: .infinity)

      VStack(alignment: .leading, spacing: 0) {
        VisualisationView(oscClient: oscClient)
        tabsRow
        SceneSliderView().padding(PADDING)
        tracksPanel
        SequenceView()
        tracks
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

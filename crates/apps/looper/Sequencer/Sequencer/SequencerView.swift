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
    @State
    var selectedTrack: Int = 1
    @State
    var selectedTab: String = "Source"


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
        let tracksPanelContentView = HStack {
          Text("Content")
        }.frame(maxWidth: .infinity)

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
        }
      }
        .padding(PADDING)
        .background(SequencerColors.black0)
        .frame(maxWidth: .infinity)

      let sequence = HStack {
        ForEach(0..<16) { i in
          let isBeat = i % 4 == 0
          Button(
            action: {
              print("\(i) clicked")
            },
            label: {
              Text("")
                .frame(maxWidth: .infinity, maxHeight: 50, alignment: .center)
                .contentShape(Rectangle())
                .foregroundColor(SequencerColors.white)
                .overlay(
                  RoundedRectangle(cornerRadius: BORDER_RADIUS)
                    .stroke(SequencerColors.black3, lineWidth: 1.0)
                )
                .background(
                  isBeat ? SequencerColors.black1 : SequencerColors.black
                )
                .cornerRadius(BORDER_RADIUS)
            }
          )
        }
      }
      .padding(PADDING)
      .background(SequencerColors.black0)
      .frame(maxWidth: .infinity)

      let visualization = ZStack {
        Rectangle()
          .fill(SequencerColors.black1)
          .frame(maxWidth: .infinity, maxHeight: .infinity)
        Rectangle()
          .fill(SequencerColors.black)
          .cornerRadius(BORDER_RADIUS)
          .frame(maxWidth: .infinity, maxHeight: .infinity)
          .padding(PADDING)
      }.frame(maxHeight: 400)

      VStack(spacing: 0) {
        visualization
        tabsRow
        tracksPanel
        sequence
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
          .background(SequencerColors.black)
          .foregroundColor(SequencerColors.white)
          .overlay(
            RoundedRectangle(cornerRadius: BORDER_RADIUS)
              .stroke(
                isSelected ? SequencerColors.red : SequencerColors.black3,
                lineWidth: 1.0
              )
          )
          .cornerRadius(BORDER_RADIUS)
      }
    )
  }
}

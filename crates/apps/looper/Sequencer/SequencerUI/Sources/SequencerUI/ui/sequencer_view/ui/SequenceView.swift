//
//  SwiftUIView.swift
//  
//
//  Created by Pedro Tacla Yamada on 11/3/2022.
//

import SwiftUI

struct SequenceModel {
  var activeSteps: Set<Int>
}

struct SequenceView: View {
  @ObservedObject var track: TrackState

  var body: some View {
    HStack {
      ForEach(0..<16) { i in
        let isActive = track.steps.contains(i)
        let isBeat = i % 4 == 0
        Button(
          action: { track.onClickStep(i) },
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
                isActive
                ? SequencerColors.blue
                : isBeat ? SequencerColors.black1 : SequencerColors.black
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
  }
}

struct SequenceView_Previews: PreviewProvider {
    static var previews: some View {
      SequenceView(track: TrackState(id: 1))
    }
}

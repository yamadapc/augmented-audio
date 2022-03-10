//
//  SwiftUIView.swift
//  
//
//  Created by Pedro Tacla Yamada on 11/3/2022.
//

import SwiftUI

struct TabsRowView: View {
  var selectedTab: String
  var onSelectTab: (String) -> Void
  var tabs = [
    "Mix",
    "Source",
    "Slice",
    "Envelope",
    "FX",
    "LFOs",
  ]

  var body: some View {
    HStack {
      ForEach(tabs, id: \.self) { tab in
        let isSelected = tab == selectedTab
        Button(
          action: {
            onSelectTab(tab)
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
  }
}

struct SwiftUIView_Previews: PreviewProvider {
  static var previews: some View {
    TabsRowView(selectedTab: "Source", onSelectTab: { _ in })
  }
}

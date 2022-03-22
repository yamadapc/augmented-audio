//
//  SwiftUIView.swift
//
//
//  Created by Pedro Tacla Yamada on 11/3/2022.
//

import SwiftUI

func renderTabValue(_ tab: TabValue) -> String {
    switch tab {
    case .mix:
        return "Mix"
    case .source:
        return "Source"
    case .slice:
        return "Slice"
    case .envelope:
        return "Envelope"
    case .fx:
        return "FX"
    case .lfos:
        return "LFOs"
    }
}

struct TabsRowView: View {
    var selectedTab: TabValue
    var onSelectTab: (TabValue) -> Void
    var tabs: [TabValue] = [
        .mix,
        .source,
        // .slice,
        .envelope,
        // .fx,
        // .lfos,
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
                        Text("\(renderTabValue(tab))")
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
        TabsRowView(selectedTab: .source, onSelectTab: { _ in })
    }
}

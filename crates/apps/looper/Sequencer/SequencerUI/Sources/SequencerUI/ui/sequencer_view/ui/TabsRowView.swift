// = copyright ====================================================================
// Continuous: Live-looper and performance sampler
// Copyright (C) 2022  Pedro Tacla Yamada
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
// = /copyright ===================================================================
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
        .slice,
        .envelope,
        // .fx,
        .lfos,
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

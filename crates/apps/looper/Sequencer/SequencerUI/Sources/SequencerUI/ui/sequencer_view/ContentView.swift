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
//  ContentView.swift
//  Sequencer
//
//  Created by Pedro Tacla Yamada on 28/2/2022.
//

import SwiftUI

/**
 * Holds the whole application GUI
 */
public struct ContentView: View {
    public init() {}

    public var body: some View {
        let view = SequencerView()
            .background(SequencerColors.black1)
            .frame(minWidth: 900, maxWidth: .infinity, minHeight: 900, maxHeight: .infinity)
            .frame(idealWidth: 1000, idealHeight: 900)

        ZStack {
            if #available(macOS 11.0, *) {
                view.preferredColorScheme(.dark)
            } else {
                view
            }
        }
        #if os(macOS)
        .fixedSize()
        #endif
    }
}

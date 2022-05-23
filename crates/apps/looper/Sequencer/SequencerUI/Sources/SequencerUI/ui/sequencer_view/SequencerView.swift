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

import Logging
import SwiftUI

let PADDING: Double = 10
let BORDER_RADIUS: Double = 8
let SHADOW_RADIUS: Double = 3

struct SequencerView: View {
    @EnvironmentObject var store: Store
    @State var dropController: DropController?

    var body: some View {
        let view = VStack(alignment: .leading, spacing: 0) {
            TopBarView()
                .bindToNilParameter(store: store)

            SequencerContentView()

            StatusBarView()
                .bindToNilParameter(store: store)
        }
        .bindToNilParameter(store: store)
        .foregroundColor(SequencerColors.white)
        .overlay(KeyboardShortcutsView(store: store))
        .overlay(GlobalOverlays())
        .setupCopyPasteController(store: store)
        .onAppear {
            dropController = DropController(store: store)
        }

        if #available(macOS 11.0, *), dropController != nil {
            view
                .onDrop(of: [.fileURL], delegate: dropController!)
        } else {
            view
        }
    }
}

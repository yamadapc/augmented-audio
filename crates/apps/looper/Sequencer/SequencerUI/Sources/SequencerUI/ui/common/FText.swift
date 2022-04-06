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
import SwiftUI

#if os(macOS)
    /**
     * The text
     * f*ast text
     */
    struct FText: NSViewRepresentable {
        typealias NSViewType = NSTextView
        var text: String

        func makeNSView(context _: Context) -> NSTextView {
            let view = NSTextView()
            view.isEditable = false
            view.isSelectable = false
            view.drawsBackground = false
            view.string = text
            return view
        }

        func updateNSView(_ nsView: NSTextView, context _: Context) {
            nsView.string = text
        }
    }
#endif

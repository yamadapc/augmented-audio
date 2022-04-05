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

#if os(macOS)
import SwiftUI

/**
     * Apparently onHover {} doesn't work properly during drag. This is a more reliable implementation for our use-case.
     */
struct CustomHoverView: NSViewRepresentable {
    typealias NSViewType = CustomHoverViewInner
    var onHover: (Bool) -> Void

    func makeNSView(context _: Context) -> CustomHoverViewInner {
        let view = CustomHoverViewInner()
        view.onHover = onHover

        let trackingArea = NSTrackingArea(
                rect: view.frame,
                options: [
                    .activeAlways,
                    .inVisibleRect,
                    .mouseEnteredAndExited,
                    .enabledDuringMouseDrag,
                ],
                owner: view,
                userInfo: [:]
        )
        view.addTrackingArea(trackingArea)
        return view
    }

    static func dismantleNSView(_ nsView: NSView, coordinator _: ()) {
        nsView.trackingAreas.forEach { trackingArea in
            nsView.removeTrackingArea(trackingArea)
        }
    }

    func updateNSView(_ nsView: CustomHoverViewInner, context _: Context) {
        nsView.onHover = onHover
    }

    class CustomHoverViewInner: NSView {
        var onHover: ((Bool) -> Void)?

        override func mouseEntered(with _: NSEvent) {
            DispatchQueue.main.async {
                self.onHover?(true)
            }
        }

        override func mouseExited(with _: NSEvent) {
            DispatchQueue.main.async {
                self.onHover?(false)
            }
        }
    }
}
#endif

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

    import Cocoa
    import KeyboardShortcuts

    @available(macOS 11.0, *)
    struct KeyWatcher: NSViewRepresentable {
        typealias NSViewType = NSView

        let onKeyDown: (KeyboardShortcuts.Key, NSEvent.ModifierFlags) -> Void
        let onKeyUp: (KeyboardShortcuts.Key, NSEvent.ModifierFlags) -> Void

        func makeNSView(context _: Context) -> NSView {
            let view = KeyWatcherNSView()
            view.onKeyDown = onKeyDown
            view.onKeyUp = onKeyUp

            DispatchQueue.main.async {
                view.window?.makeFirstResponder(view)
            }

            return view
        }

        func updateNSView(_: NSView, context _: Context) {}
    }

    @available(macOS 11.0, *)
    private class KeyWatcherNSView: NSView {
        var onKeyDown: ((KeyboardShortcuts.Key, NSEvent.ModifierFlags) -> Void)?
        var onKeyUp: ((KeyboardShortcuts.Key, NSEvent.ModifierFlags) -> Void)?

        override func keyDown(with event: NSEvent) {
            let key = KeyboardShortcuts.Key(rawValue: Int(event.keyCode))
            let modifiers = event.modifierFlags
            onKeyDown?(key, modifiers)
        }

        override func keyUp(with event: NSEvent) {
            let key = KeyboardShortcuts.Key(rawValue: Int(event.keyCode))
            let modifiers = event.modifierFlags
            onKeyUp?(key, modifiers)
        }
    }

#endif

struct KeyboardShortcutsView: View {
    var store: Store

    #if os(macOS)
        @State var controller: KeyboardShortcutsController

        init(store: Store) {
            self.store = store
            controller = KeyboardShortcutsController(store: store)
        }
    #endif

    var body: some View {
        ZStack {
            #if os(macOS)
                if #available(macOS 11.0, *) {
                    KeyWatcher(
                        onKeyDown: { key, modifiers in
                            controller.onKeyDown(
                                key: key,
                                modifiers: modifiers
                            )
                        },
                        onKeyUp: { key, modifiers in
                            controller.onKeyUp(
                                key: key,
                                modifiers: modifiers
                            )
                        }
                    )
                } else {}
            #endif
        }.allowsHitTesting(false)
    }
}

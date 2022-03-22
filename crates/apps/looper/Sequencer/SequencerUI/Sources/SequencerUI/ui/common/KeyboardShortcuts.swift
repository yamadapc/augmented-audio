#if os(macOS)

    import Cocoa
    import SwiftUI

    import KeyboardShortcuts

    @available(macOS 11.0, *)
    struct KeyWatcher: NSViewRepresentable {
        typealias NSViewType = NSView

        let onEvent: (KeyboardShortcuts.Key, NSEvent.ModifierFlags) -> Void

        func makeNSView(context _: Context) -> NSView {
            let view = KeyWatcherNSView()
            view.onEvent = onEvent

            DispatchQueue.main.async {
                view.window?.makeFirstResponder(view)
            }

            return view
        }

        func updateNSView(_: NSView, context _: Context) {}
    }

    @available(macOS 11.0, *)
    private class KeyWatcherNSView: NSView {
        var onEvent: ((KeyboardShortcuts.Key, NSEvent.ModifierFlags) -> Void)?

        override func keyDown(with event: NSEvent) {
            let key = KeyboardShortcuts.Key(rawValue: Int(event.keyCode))
            let modifiers = event.modifierFlags
            onEvent?(key, modifiers)
        }
    }

#endif

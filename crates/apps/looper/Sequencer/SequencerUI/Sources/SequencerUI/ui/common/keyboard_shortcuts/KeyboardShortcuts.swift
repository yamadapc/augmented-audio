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

func buildKeyWatcher(store _: Store) -> some View {
    ZStack {
        #if os(macOS)
            if #available(macOS 11.0, *) {
                KeyWatcher(
                    onKeyDown: { key, modifiers in
                        print("KEYDOWN - key=\(key) modifiers=\(modifiers)")
                    },
                    onKeyUp: { key, modifiers in
                        print("KEYUP - key=\(key) modifiers=\(modifiers)")
                    }
                )
            } else {}
        #endif
    }.allowsHitTesting(false)
}

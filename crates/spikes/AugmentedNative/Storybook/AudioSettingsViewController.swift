//
//  AudioSettingsViewController.swift
//  Storybook
//
//  Created by Pedro Tacla Yamada on 23/8/21.
//

import Cocoa
import SwiftUI

func setupWindow() -> NSWindow {
    // Create the window and set the content view.
    let window = NSWindow(
        contentRect: NSRect(x: 0, y: 0, width: 480, height: 300),
        styleMask: [.titled, .closable, .miniaturizable, .resizable, .fullSizeContentView],
        backing: .buffered,
        defer: false
    )
    window.isReleasedWhenClosed = false
    window.center()
    window.setFrameAutosaveName("Main Window")
    window.makeKeyAndOrderFront(nil)
    return window
}

@available(macOS 11.0, *)
func setupContentView() -> NSView {
    let contentView = ContentView()
    return NSHostingView(rootView: contentView)
}

//
//  AppDelegate.swift
//  Recording Buddy
//
//  Created by Pedro Tacla Yamada on 27/8/21.
//

import Cocoa
import SwiftUI
import RecordingBuddyViews

@main
class AppDelegate: NSObject, NSApplicationDelegate {
    var appContext = AppContextImpl()
    var audioOptionsService = AudioOptionsService()
    var statusItem: NSStatusItem!
    var window: NSWindow!

    func applicationDidFinishLaunching(_ aNotification: Notification) {
        statusItem = NSStatusBar.system.statusItem(withLength: NSStatusItem.squareLength)
        if let button = statusItem.button {
            button.image = NSImage(named: NSImage.Name("MenuBarIcon"))
            button.action = #selector(onClickStatusItem(_:))
        }

        // Create the SwiftUI view that provides the window contents.
        let contentView = ContentView(
            engineStateViewModel: EngineStateViewModel(isRunning: true)
        ).environment(\.appContext, appContext)

        // Create the window and set the content view.
        window = NSWindow(
            contentRect: NSRect(x: 0, y: 0, width: 480, height: 300),
            styleMask: [.titled, .closable, .miniaturizable, .resizable, .fullSizeContentView],
            backing: .buffered, defer: false)
        window.isReleasedWhenClosed = false
        window.setFrameAutosaveName("Main Window")
        window.contentView = NSHostingView(rootView: contentView)

        // TODO - This should only happen during development
        window.center()
        window.makeKeyAndOrderFront(self)
    }

    func applicationWillTerminate(_ aNotification: Notification) {
        // Insert code here to tear down your application
    }

    @objc
    func onClickStatusItem(_ sender: Any?) {
        let menuItemFrame = NSApp.currentEvent!.window!.frame
        let menuItemOrigin = menuItemFrame.origin
        let origin = menuItemOrigin.applying(
            CGAffineTransform.init(translationX: 0.0, y: -self.window.frame.height)
        )

        self.window.setFrameOrigin(origin)
        NSApp.activate(ignoringOtherApps: true)
        self.window.makeKeyAndOrderFront(sender)
    }
}

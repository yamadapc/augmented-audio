//
//  AppDelegate.swift
//  Recording Buddy
//
//  Created by Pedro Tacla Yamada on 27/8/21.
//

import Cocoa
import SwiftUI
import RecordingBuddyViews

let MENU_BAR_ICON = "MenuBarIcon"
let DEFAULT_WIDTH = 480
let DEFAULT_HEIGHT = 300

func getWindowStyleMask() -> NSWindow.StyleMask {
    #if DEBUG
    return [.titled, .closable, .miniaturizable, .resizable, .fullSizeContentView]
    #else
    return [.fullSizeContentView, .utilityWindow]
    #endif
}

@available(macOS 11.0, *)
@main
class AppDelegate: NSObject, NSApplicationDelegate {
    var appContext = AppContextImpl()
    var audioOptionsService = AudioOptionsService()
    var statusItem: NSStatusItem!
    var window: NSWindow!

    func applicationDidFinishLaunching(_ aNotification: Notification) {
        statusItem = NSStatusBar.system.statusItem(withLength: NSStatusItem.squareLength)
        if let button = statusItem.button {
            button.image = NSImage(named: NSImage.Name(MENU_BAR_ICON))
            button.action = #selector(onClickStatusItem(_:))
        }

        // Create the SwiftUI view that provides the window contents.
        let contentView = ContentView(
            engineStateViewModel: appContext.engineState()
        ).environment(\.appContext, appContext)

        // Create the window and set the content view.
        window = NSWindow(
            contentRect: NSRect(x: 0, y: 0, width: DEFAULT_WIDTH, height: DEFAULT_HEIGHT),
            styleMask: getWindowStyleMask(),
            backing: .buffered,
            defer: false
        )
        window.isReleasedWhenClosed = false
        window.title = "Recording Buddy"
        window.setFrameAutosaveName("Main Window")
        window.contentView = NSHostingView(rootView: contentView)

        #if DEBUG
        window.center()
        window.makeKeyAndOrderFront(self)
        appContext.navigationDelegate().navigate(.openSettings)
        #endif
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

        self.window.hidesOnDeactivate = true
        self.window.setFrameOrigin(origin)
        NSApp.activate(ignoringOtherApps: true)
        self.window.makeKeyAndOrderFront(sender)
    }
}

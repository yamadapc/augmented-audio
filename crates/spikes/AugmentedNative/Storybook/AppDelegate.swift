//
//  AppDelegate.swift
//  Storybook
//
//  Created by Pedro Tacla Yamada on 22/8/21.
//

import Cocoa
import SwiftUI
import MetalKit

@available(macOS 11.0, *)
@main
class AppDelegate: NSObject, NSApplicationDelegate {
    var window: NSWindow!
    var metalWindow: NSWindow!

    func applicationDidFinishLaunching(_ aNotification: Notification) {
        initializeLogger()

        window = setupWindow(width: 300, height: 300)
        window.title = "Audio Settings"
        window.contentView = setupContentView()

        metalWindow = setupWindow(width: 600, height: 600)
        metalWindow.title = "Metal"
        let metalView = NSView()
        metalWindow.contentView = metalView
        run_loop(Unmanaged.passRetained(metalView).toOpaque())
    }

    func applicationWillTerminate(_ aNotification: Notification) {
        // Insert code here to tear down your application
    }
}


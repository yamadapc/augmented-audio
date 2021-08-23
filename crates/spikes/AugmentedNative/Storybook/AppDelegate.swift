//
//  AppDelegate.swift
//  Storybook
//
//  Created by Pedro Tacla Yamada on 22/8/21.
//

import Cocoa
import SwiftUI

@available(macOS 11.0, *)
@main
class AppDelegate: NSObject, NSApplicationDelegate {
    var window: NSWindow!

    func applicationDidFinishLaunching(_ aNotification: Notification) {
        initializeLogger()

        window = setupWindow()
        window.contentView = setupContentView()
    }

    func applicationWillTerminate(_ aNotification: Notification) {
        // Insert code here to tear down your application
    }
}


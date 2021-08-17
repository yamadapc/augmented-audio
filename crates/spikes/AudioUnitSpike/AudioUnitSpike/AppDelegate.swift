//
//  AppDelegate.swift
//  AudioUnitSpike
//
//  Created by Pedro Tacla Yamada on 18/8/21.
//

import Cocoa
import SwiftUI
import AVFAudio

public func listAllAudioUnits() -> [String] {
    let manager = AVAudioUnitComponentManager.shared()
    let components = manager.components(matching: NSPredicate { _, _ in true })
    return components.map { component in
        component.name
    }
}

@main
class AppDelegate: NSObject, NSApplicationDelegate {

    var window: NSWindow!


    func applicationDidFinishLaunching(_ aNotification: Notification) {
        let _ = listAllAudioUnits()

        // Create the SwiftUI view that provides the window contents.
        let contentView = ContentView()

        // Create the window and set the content view.
        window = NSWindow(
            contentRect: NSRect(x: 0, y: 0, width: 480, height: 300),
            styleMask: [.titled, .closable, .miniaturizable, .resizable, .fullSizeContentView],
            backing: .buffered, defer: false)
        window.isReleasedWhenClosed = false
        window.center()
        window.setFrameAutosaveName("Main Window")
        window.contentView = NSHostingView(rootView: contentView)
        window.makeKeyAndOrderFront(nil)
    }

    func applicationWillTerminate(_ aNotification: Notification) {
        // Insert code here to tear down your application
    }


}


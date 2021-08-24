//
//  AppDelegate.swift
//  Storybook
//
//  Created by Pedro Tacla Yamada on 22/8/21.
//

import Cocoa
import SwiftUI
import OpenGL

class GLView: NSOpenGLView {
    override func draw(_ dirtyRect: NSRect) {
        run_gl_draw()
    }
}

@available(macOS 11.0, *)
@main
class AppDelegate: NSObject, NSApplicationDelegate {
    var window: NSWindow!
    var openGLWindow: NSWindow!

    func applicationDidFinishLaunching(_ aNotification: Notification) {
        initializeLogger()

        window = setupWindow(width: 300, height: 300)
        window.title = "Audio Settings"
        window.contentView = setupContentView()

        openGLWindow = setupWindow(width: 600, height: 600)
        openGLWindow.title = "OpenGL"
        let openGLView = GLView()
        openGLWindow.contentView = openGLView
        let openGLContext = openGLView.openGLContext
        let cglContext = openGLContext?.cglContextObj
        run_gl_loop(cglContext)
    }

    func applicationWillTerminate(_ aNotification: Notification) {
        // Insert code here to tear down your application
    }
}


import Cocoa
import FlutterMacOS

@NSApplicationMain
class AppDelegate: FlutterAppDelegate {
    var statusItem: NSStatusItem!

    override func applicationDidFinishLaunching(_ notification: Notification) {
        statusItem = NSStatusBar.system.statusItem(withLength: NSStatusItem.squareLength)
        if let button = statusItem.button {
            button.image = NSImage(named: NSImage.Name("MenuBarIcon"))
            button.action = #selector(onClickMenuBarItem(_:))
        }
    }

    override func applicationShouldTerminateAfterLastWindowClosed(_ sender: NSApplication) -> Bool {
        return false
    }

    override func applicationDidResignActive(_ notification: Notification) {
        self.mainFlutterWindow.orderOut(self)
    }

    @objc
    func onClickMenuBarItem(_ sender: Any?) {
        let menuItemFrame = NSApp.currentEvent!.window!.frame
        let menuItemOrigin = menuItemFrame.origin
        let menuItemHeight = menuItemFrame.height
        let origin = menuItemOrigin.applying(
            CGAffineTransform.init(translationX: 0.0, y: -self.mainFlutterWindow.frame.height)
        )
        print("Origin: \(origin) \(menuItemOrigin) \(menuItemHeight)")
        self.mainFlutterWindow.setFrameOrigin(origin)
        self.mainFlutterWindow.makeKeyAndOrderFront(sender)
    }
}

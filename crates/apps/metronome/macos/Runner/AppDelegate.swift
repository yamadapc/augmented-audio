import Cocoa
import Firebase
import FlutterMacOS

@NSApplicationMain
class AppDelegate: FlutterAppDelegate {
  override func applicationDidFinishLaunching(_ notification: Notification) {
    let result = dummy_method_to_enforce_bundling()
    FirebaseApp.configure()
    print("Initialized \(result)")
  }

  override func applicationShouldTerminateAfterLastWindowClosed(_ sender: NSApplication) -> Bool {
    return true
  }
}

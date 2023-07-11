import Cocoa
import FlutterMacOS

@NSApplicationMain
class AppDelegate: FlutterAppDelegate {
  override func applicationDidFinishLaunching(_ notification: Notification) {
    let result = dummy_method_to_enforce_bundling()
    print("Initialized \(result)")
  }

  override func applicationWillTerminate(_ notification: Notification) {
    metronome_will_terminate()
  }

  override func applicationShouldTerminateAfterLastWindowClosed(_ sender: NSApplication) -> Bool {
    return true
  }
}

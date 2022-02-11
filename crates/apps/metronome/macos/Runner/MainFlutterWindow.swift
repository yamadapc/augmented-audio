import Cocoa
import FlutterMacOS

class MainFlutterWindow: NSWindow {
  override func awakeFromNib() {
    let flutterViewController = FlutterViewController.init()
    self.contentViewController = flutterViewController

    let size = CGSize(width: 300.0, height: 600.0)
    let windowFrame = NSRect(
      origin: self.frame.origin,
      size: size
    )
    self.minSize = size
    self.setFrame(
      windowFrame,
      display: true
    )
    self.styleMask = self.styleMask.union(StyleMask.resizable)

    RegisterGeneratedPlugins(registry: flutterViewController)

    super.awakeFromNib()
  }
}

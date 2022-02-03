import Cocoa
import FlutterMacOS

class MainFlutterWindow: NSWindow {
  override func awakeFromNib() {
    let flutterViewController = FlutterViewController.init()
    self.contentViewController = flutterViewController

    let size = CGSize(width: 300.0, height: 600.0)
    self.minSize = size
    // self.maxSize = CGSize(width: 900.0, height: 900.0)
    self.setFrame(
      NSRect(
        origin: self.frame.origin,
        size: size
      ),
      display: true
    )
    self.styleMask = self.styleMask.union(StyleMask.resizable)

    RegisterGeneratedPlugins(registry: flutterViewController)

    super.awakeFromNib()
  }
}

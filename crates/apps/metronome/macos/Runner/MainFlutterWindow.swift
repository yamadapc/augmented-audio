import Cocoa
import FlutterMacOS

class MainFlutterWindow: NSWindow {
  override func awakeFromNib() {
    let flutterViewController = FlutterViewController.init()
    self.contentViewController = flutterViewController

    let size = CGSize(width: 300.0, height: 180.0)
    self.minSize = size
    self.maxSize = size
    self.setFrame(
      NSRect(
        origin: self.frame.origin,
        size: size
      ),
      display: true
    )

    RegisterGeneratedPlugins(registry: flutterViewController)

    super.awakeFromNib()
  }
}

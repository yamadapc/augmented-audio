import SwiftUI

#if os(macOS)
  struct AudioPathMetalView: NSViewRepresentable {
      typealias NSViewType = NSView

      var layer: CAMetalLayer?
      var size: CGSize
      var nsView: NSView?

      func makeNSView(context: Context) -> NSView {
          let view = NSView()
          view.wantsLayer = true
          view.layer = layer!
          view.frame.size = size
          layer?.drawableSize = size
          return view
      }

      func updateNSView(_ nsView: NSView, context: Context) {
          nsView.layer = layer
          nsView.frame.size = size
          layer?.drawableSize = nsView.frame.size
      }
  }
#endif

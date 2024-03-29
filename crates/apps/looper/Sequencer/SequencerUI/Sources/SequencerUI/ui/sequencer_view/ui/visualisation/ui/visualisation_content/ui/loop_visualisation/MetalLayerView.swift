// = copyright ====================================================================
// Continuous: Live-looper and performance sampler
// Copyright (C) 2022  Pedro Tacla Yamada
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
// = /copyright ===================================================================

import MetalKit
import SwiftUI
import Logging

fileprivate let logger = Logger(label: "com.beijaflor.sequencer.engine.MetalLayerView")

private var metalViewDelegate = CALayerBackedView.CALayerBackedUIViewDelegate()

class CALayerBackedView: MTKView {
    #if os(iOS)
    override class var layerClass: AnyClass {
        CAMetalLayer.self
    }
    #endif
    var drawFn: ((CAMetalLayer) -> Void)?

    class CALayerBackedUIViewDelegate: NSObject, MTKViewDelegate {
        func mtkView(_: MTKView, drawableSizeWillChange _: CGSize) {
            //    let view = view as! CALayerBackedView
            //    let layer = view.layer as! CAMetalLayer
            //    layer.drawableSize = size
            //    view.drawFn?(layer)
        }

        func draw(in view: MTKView) {
            let view = view as! CALayerBackedView
            let layer = view.layer as! CAMetalLayer
            layer.drawableSize = view.frame.size
            view.drawFn?(layer)
            #if os(iOS)
                view.setNeedsDisplay()
            #endif
        }
    }
}

#if os(macOS)
    struct AudioPathMetalView: NSViewRepresentable {
        typealias NSViewType = CALayerBackedView

        var size: CGSize
        var draw: ((CAMetalLayer) -> Void)?

        func makeNSView(context _: Context) -> CALayerBackedView {
            let view = CALayerBackedView()
            view.frame.size = size
            view.drawFn = draw
            view.delegate = metalViewDelegate
            let metalLayer = view.layer as! CAMetalLayer
            AudioPathMetalView.updateMetalLayer(view, metalLayer)
            return view
        }

        func updateNSView(_ view: CALayerBackedView, context _: Context) {
            view.frame.size = size
            view.drawFn = draw
            let metalLayer = view.layer as! CAMetalLayer
            AudioPathMetalView.updateMetalLayer(view, metalLayer)
        }

        // We need to respect the scaling factor for the current display we are in

        private static func updateMetalLayer(_ view: NSView, _ metalLayer: CAMetalLayer) {
            let scalingFactor = NSScreen.main?.backingScaleFactor ?? 1.0
            logger.info("Update NSView - using scaling factor \(scalingFactor)")
            metalLayer.pixelFormat = .bgra8Unorm
            metalLayer.contentsScale = scalingFactor
            metalLayer.drawableSize = view.frame.size.applying(.init(scaleX: scalingFactor, y: scalingFactor))
            metalLayer.frame = view.frame
        }
    }

#elseif os(iOS)
    struct AudioPathMetalView: UIViewRepresentable {
        typealias UIViewType = CALayerBackedView

        var size: CGSize
        var draw: ((CAMetalLayer) -> Void)?

        func makeUIView(context _: Context) -> CALayerBackedView {
            let view = CALayerBackedView()
            view.frame.size = size
            view.drawFn = draw
            view.delegate = metalViewDelegate
            let metalLayer = view.layer as! CAMetalLayer
            metalLayer.drawableSize = view.frame.size
            metalLayer.frame = view.frame
            return view
        }

        func updateUIView(_ view: CALayerBackedView, context _: Context) {
            view.frame.size = size
            view.drawFn = draw
            let metalLayer = view.layer as! CAMetalLayer
            metalLayer.drawableSize = view.frame.size
            metalLayer.frame = view.frame
        }
    }
#endif

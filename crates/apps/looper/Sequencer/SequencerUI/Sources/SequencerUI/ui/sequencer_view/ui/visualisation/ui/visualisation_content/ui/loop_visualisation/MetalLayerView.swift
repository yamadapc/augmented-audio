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

var caLayerBackedUIViewDelegate = CALayerBackedView.CALayerBackedUIViewDelegate()
class CALayerBackedView: MTKView {
    #if os(iOS)
        override class var layerClass: AnyClass {
            CAMetalLayer.self
        }
    #endif
    var drawFn: ((CAMetalLayer) -> Void)?

    class CALayerBackedUIViewDelegate: NSObject, MTKViewDelegate {
        func mtkView(_ view: MTKView, drawableSizeWillChange size: CGSize) {
            let view = view as! CALayerBackedView
            let layer = view.layer as! CAMetalLayer
            layer.drawableSize = size
            view.drawFn?(layer)
            #if os(iOS)
                view.setNeedsDisplay()
            #endif
        }

        func draw(in view: MTKView) {
            let view = view as! CALayerBackedView
            let layer = view.layer as! CAMetalLayer
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
            view.delegate = caLayerBackedUIViewDelegate
            let metalLayer = view.layer as! CAMetalLayer
            metalLayer.drawableSize = view.frame.size
            return view
        }

        func updateNSView(_ view: CALayerBackedView, context _: Context) {
            view.frame.size = size
            view.drawFn = draw
            let metalLayer = view.layer as! CAMetalLayer
            metalLayer.drawableSize = view.frame.size
        }
    }

#elseif os(iOS)
    struct AudioPathMetalView: UIViewRepresentable {
        typealias UIViewType = CALayerBackedUIView

        var size: CGSize
        var draw: ((CAMetalLayer) -> Void)?

        func makeUIView(context _: Context) -> CALayerBackedUIView {
            let view = CALayerBackedUIView()
            view.frame.size = size
            view.drawFn = draw
            view.delegate = caLayerBackedUIViewDelegate
            let metalLayer = view.layer as! CAMetalLayer
            metalLayer.drawableSize = view.frame.size
            return view
        }

        func updateUIView(_ view: CALayerBackedUIView, context _: Context) {
            view.frame.size = size
            view.drawFn = draw
            let metalLayer = view.layer as! CAMetalLayer
            metalLayer.drawableSize = view.frame.size
        }
    }
#endif

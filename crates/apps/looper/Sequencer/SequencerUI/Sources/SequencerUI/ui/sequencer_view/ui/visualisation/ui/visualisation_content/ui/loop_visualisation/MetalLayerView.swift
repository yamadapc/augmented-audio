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

import SwiftUI
import MetalKit

#if os(macOS)
    struct AudioPathMetalView: NSViewRepresentable {
        typealias NSViewType = NSView

        var layer: CAMetalLayer?
        var size: CGSize
        var onLayer: ((CAMetalLayer) -> Void)?

        func makeNSView(context _: Context) -> NSView {
            let view = NSView()
            view.wantsLayer = true
            view.layer = layer!
            view.frame.size = size
            layer?.drawableSize = size
            return view
        }

        func updateNSView(_ view: NSView, context _: Context) {
            view.layer = layer
            view.frame.size = size
            layer?.drawableSize = view.frame.size
        }
    }

#elseif os(iOS)
class CALayerBackedUIView: MTKView {
    override class var layerClass: AnyClass {
        CAMetalLayer.self
    }
    var drawFn: ((CAMetalLayer) -> Void)?

    class CALayerBackedUIViewDelegate: NSObject, MTKViewDelegate {
        func mtkView(_ view: MTKView, drawableSizeWillChange size: CGSize) {
        }

        func draw(in view: MTKView) {
            let view = view as! CALayerBackedUIView
            let layer = view.layer as! CAMetalLayer
            view.drawFn?(layer)
            view.setNeedsDisplay()
        }
    }
}

var caLayerBackedUIViewDelegate = CALayerBackedUIView.CALayerBackedUIViewDelegate()

struct AudioPathMetalView: UIViewRepresentable {
    typealias UIViewType = CALayerBackedUIView

    var layer: CAMetalLayer?
    var size: CGSize
    var draw: ((CAMetalLayer) -> Void)?

    func makeUIView(context _: Context) -> CALayerBackedUIView {
        let view = CALayerBackedUIView()
        view.frame.size = size
        view.delegate = caLayerBackedUIViewDelegate
        let metalLayer = view.layer as! CAMetalLayer
        metalLayer.drawableSize = view.frame.size
        view.drawFn = draw
        return view
    }

    func updateUIView(_ view: CALayerBackedUIView, context _: Context) {
        view.frame.size = size
        let metalLayer = view.layer as! CAMetalLayer
        metalLayer.drawableSize = view.frame.size
        view.drawFn = draw
    }
    }
#endif

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

import AVKit
import LRUCache
import MetalKit

#if !os(iOS)
    import Cocoa
    import LRUCache

    struct MTKVisualizationView: NSViewRepresentable {
        typealias NSViewType = MTKView
        var mtkView = MTKView()

        func makeCoordinator() -> Coordinator {
            return Coordinator(self, mtkView)
        }

        func makeNSView(context: Context) -> MTKView {
            mtkView.delegate = context.coordinator
            mtkView.enableSetNeedsDisplay = true
            return mtkView
        }

        func updateNSView(_: MTKView, context _: Context) {}

        class Coordinator: NSObject, MTKViewDelegate {
            var parent: MTKVisualizationView
            var view: MTKView

            var metalDevice: MTLDevice!
            var metalCommandQueue: MTLCommandQueue!

            init(_ parent: MTKVisualizationView, _ view: MTKView) {
                self.parent = parent
                self.view = view
                if let metalDevice = MTLCreateSystemDefaultDevice() {
                    view.device = metalDevice
                    self.metalDevice = metalDevice
                }
                metalCommandQueue = metalDevice.makeCommandQueue()!
                super.init()
            }

            func mtkView(_: MTKView, drawableSizeWillChange _: CGSize) {}

            func draw(in view: MTKView) {
                guard let drawable = view.currentDrawable else {
                    return
                }
                let commandBuffer = metalCommandQueue.makeCommandBuffer()
                var size = view.bounds
                size.size = view.drawableSize

                commandBuffer?.present(drawable)
                commandBuffer?.commit()
            }
        }
    }
#endif

// This is technically bad as it will cache across heights
let PATH_CACHE: LRUCache<Int, CGPath> = LRUCache(
    totalCostLimit: 10,
    countLimit: 10
)

func buildCacheKey(_ geometry: GeometryProxy, _ buffer: TrackBuffer) -> Int {
    var hash = Hasher()
    hash.combine(buffer.id)
    hash.combine(geometry.size.height)
    hash.combine(geometry.size.width.hashValue)
    for sample in 0 ..< buffer.count {
        hash.combine(buffer[sample].hashValue)
    }
    return hash.finalize()
}

func buildPath(_ geometry: GeometryProxy, _ buffer: TrackBuffer) -> CGPath {
    let cacheKey = buildCacheKey(geometry, buffer)
    if let cachedPath = PATH_CACHE.value(forKey: cacheKey) {
        return cachedPath
    }

    let path = CGMutablePath()
    let height = geometry.size.height
    let width = Int(geometry.size.width)

    if buffer.count == 0 {
        return path
    }

    var maxSample = 0.0
    for index in 0 ..< buffer.count {
        let value: Float = abs(buffer[index % buffer.count])
        maxSample = max(maxSample, Double(value))
    }

    if maxSample == 0.0 {
        return path
    }

    for index in 0 ..< buffer.count {
        let x = (Double(index) / Double(buffer.count)) * Double(width)
        let value = buffer[index]
        let ratio = Double(value) / maxSample
        let h = ratio * height / 2 + height / 2

        if index == 0 {
            path.move(to: CGPoint(x: x, y: h))
        }
        path.addLine(to: CGPoint(x: x, y: h))
    }

    PATH_CACHE.setValue(path, forKey: cacheKey)
    return path
}

public struct AudioPathView: View {
    var tick: Int
    var buffer: TrackBuffer
    var geometry: GeometryProxy
    @State
    var uiImage: UIImage? = nil

    public init(tick: Int, buffer: TrackBuffer, geometry: GeometryProxy) {
        self.tick = tick
        self.buffer = buffer
        self.geometry = geometry
    }

    public var body: some View {
        ZStack {
            if let uiImage = uiImage {
                Image(uiImage: uiImage)
                  .resizable()
                  .frame(maxWidth: .infinity, maxHeight: .infinity)
            } else {
                Text("Drawing...")
            }
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity, alignment: .center)
        .onAppear {
            DispatchQueue.global(qos: .userInitiated).async {
                let path = timeFunction("Building path") { buildPath(geometry, buffer) }
                let renderer = UIGraphicsImageRenderer(bounds: geometry.frame(in: .local), format: .init())
                self.uiImage = timeFunction("Drawing image") { renderer.image { renderContext in
                    let cgContext = renderContext.cgContext
                    cgContext.addPath(path)
                    cgContext.setLineWidth(1.0)
                    cgContext.setStrokeColor(CGColor(red: 1.0, green: 0, blue: 0, alpha: 1))
                    cgContext.strokePath()
                } }
            }
        }
    }
}

extension AudioPathView: Equatable {
    public static func == (lhs: Self, rhs: Self) -> Bool {
        lhs.buffer.equals(other: rhs.buffer) && lhs.geometry.size == rhs.geometry.size
    }
}

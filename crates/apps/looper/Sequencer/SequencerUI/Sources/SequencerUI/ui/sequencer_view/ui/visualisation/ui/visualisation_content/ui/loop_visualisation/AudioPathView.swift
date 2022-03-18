import SwiftUI

import AVKit
import MetalKit

#if !os(iOS)
    import Cocoa

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

func buildPath(_ geometry: GeometryProxy, _ path: inout Path, _: Int, _ buffer: TrackBuffer) {
    let height = geometry.size.height
    let width = Int(geometry.size.width)

    let step = Int(Double(buffer.count) / Double(width))

    var maxSample = 0.0
    for overSampledX in 0 ... (width * 2) {
        let x = Double(overSampledX) / 2.0
        let index = Int(x / Double(width) * Double(buffer.count))
        var value: Float = 0.0
        for j in 0 ..< step {
            value += abs(buffer[(index + j) % buffer.count])
        }
        value /= Float(step)

        maxSample = max(maxSample, Double(value))
    }

    for overSampledX in 0 ... (width * 2) {
        let x = Double(overSampledX) / 2.0
        let index = Int(x / Double(width) * Double(buffer.count))
        var value: Float = 0.0
        for j in 0 ..< step {
            value += buffer[(index + j) % buffer.count]
        }
        value /= Float(step)

        let h = (Double(value) / maxSample) * height / 2 + height / 2

        if overSampledX == 0 {
            path.move(to: CGPoint(x: x, y: h))
        }
        path.addLine(to: CGPoint(x: x, y: h))
    }
}

struct AudioPathView: View {
    var tick: Int
    var buffer: TrackBuffer
    var geometry: GeometryProxy

    var body: some View {
        Path { path in
            buildPath(geometry, &path, tick, buffer)
        }
        .stroke(SequencerColors.blue, lineWidth: 1)
    }
}

extension AudioPathView: Equatable {
    static func == (lhs: Self, rhs: Self) -> Bool {
        lhs.buffer.equals(other: rhs.buffer)
    }
}

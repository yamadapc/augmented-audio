//
//  SwiftUIView.swift
//
//
//  Created by Pedro Tacla Yamada on 11/3/2022.
//

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

func buildPath(_ geometry: GeometryProxy, _ path: inout Path, _: Int, _ buffer: UnsafeBufferPointer<Float32>) {
    let height = geometry.size.height
    let width = Int(geometry.size.width)

    for x in 0 ... width {
        let index = Int(Double(x) / Double(width) * Double(buffer.count))
        let value: Float = buffer[index % buffer.count] / 2.0

        let h = Double(value) * height / 2 + height / 2

        if x == 0 {
            path.move(to: CGPoint(x: Double(x), y: h))
        }
        path.addLine(to: CGPoint(x: Double(x), y: h))
    }
}

struct LoopVisualisationView: View {
    @ObservedObject var trackState: TrackState
    @State var tick: Int = 0

    var body: some View {
        if #available(macOS 12.0, *) {
            self.renderInner(tick: 0)
            // TimelineView(.periodic(from: .now, by: 1 / 30)) { timeline in
            //     self.renderInner(tick: Int(timeline.date.timeIntervalSince1970 * 1000))
            // }
        } else {
            self.renderInner(tick: 0)
        }
    }

    func renderInner(tick: Int) -> some View {
        ZStack {
            if let buffer = trackState.buffer {
                GeometryReader { geometry in
                    Path { path in
                        buildPath(geometry, &path, tick, buffer)
                    }
                    .stroke(SequencerColors.blue, lineWidth: 2)
                }
                .padding()
            } else {
                Text("No loop buffer")
            }
        }
    }
}

struct LoopVisualisationView_Previews: PreviewProvider {
    static var previews: some View {
        LoopVisualisationView(trackState: TrackState(id: 0))
            .cornerRadius(BORDER_RADIUS)
    }
}

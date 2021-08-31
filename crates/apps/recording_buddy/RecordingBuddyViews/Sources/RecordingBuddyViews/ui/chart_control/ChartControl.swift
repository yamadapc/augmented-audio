//
//  ChartControl.swift
//  Recording Buddy
//
//  Created by Pedro Tacla Yamada on 27/8/21.
//

import SwiftUI
import MetalKit

struct MetalView: NSViewRepresentable {
    typealias NSViewType = MTKView

    func makeNSView(context: Context) -> MTKView {
        return MTKView()
    }

    func updateNSView(_ nsView: MTKView, context: Context) {}
}

struct ChartControl: View {
    var body: some View {
        ZStack {
            Text("Threshold").frame(maxWidth: .infinity, maxHeight: .infinity)
            MetalView()
        }.frame(alignment: .center)
    }
}

struct ChartControl_Previews: PreviewProvider {
    static var previews: some View {
        ChartControl()
            .frame(minWidth: 300, minHeight: 200, alignment: .topLeading)
    }
}

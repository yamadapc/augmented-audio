//
//  SwiftUIView.swift
//  
//
//  Created by Pedro Tacla Yamada on 1/9/21.
//

import SwiftUI
import MetalKit

struct MTKSwiftUIView: NSViewRepresentable {
    typealias NSViewType = MTKView

    func makeNSView(context: Context) -> NSViewType {
        let view = MTKView()
        return view
    }

    func updateNSView(_ nsView: NSViewType, context: Context) {}
}

struct TestMetalView: View {
    var body: some View {
        MTKSwiftUIView()
    }
}

struct TestMetalView_Previews: PreviewProvider {
    static var previews: some View {
        TestMetalView()
    }
}

//
//  ChartControl.swift
//  Recording Buddy
//
//  Created by Pedro Tacla Yamada on 27/8/21.
//

import SwiftUI
import MetalKit

struct MetalView: NSViewRepresentable {
    typealias NSViewType = NSView

    func makeNSView(context: Context) -> NSViewType {
        let appContext = context.environment.appContext
        let view = NSView()
        view.wantsLayer = true
        DispatchQueue.main.async {
            try? appContext.chartHandler().onChartView(view)
        }
        return view
    }

    func updateNSView(_ nsView: NSViewType, context: Context) {}
}

struct ChartControl: View {
    var body: some View {
        Group {
            ZStack {
                MetalView()
            }
            .frame(alignment: .center)
            .background(
                Color(red: 0.15, green: 0.15, blue: 0.15, opacity: 1)
            )
            .foregroundColor(Color.white)
            .border(Color(NSColor.underPageBackgroundColor), width: 1)
        }
        .padding(10)
    }
}

struct ChartControl_Previews: PreviewProvider {
    static var previews: some View {
        ChartControl()
            .frame(minWidth: 300, minHeight: 200, alignment: .topLeading)
    }
}

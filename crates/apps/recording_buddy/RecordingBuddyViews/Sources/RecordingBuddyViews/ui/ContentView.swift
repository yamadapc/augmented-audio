//
//  ContentView.swift
//  Recording Buddy
//
//  Created by Pedro Tacla Yamada on 27/8/21.
//

import SwiftUI

public struct ContentView: View {
    var engineStateViewModel: EngineStateViewModel

    public init(
        engineStateViewModel: EngineStateViewModel
    ) {
        self.engineStateViewModel = engineStateViewModel
    }

    public var body: some View {
        VStack(spacing: 0) {
            EngineStateView(model: engineStateViewModel)
            // Charts are disabled for a first iteration.
            // Divider()
            // ChartControl()
            Divider()
            FooterView()
        }.frame(minWidth: 300, minHeight: 500, alignment: .topLeading)
    }
}


struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView(engineStateViewModel: EngineStateViewModel(isRunning: true))
    }
}

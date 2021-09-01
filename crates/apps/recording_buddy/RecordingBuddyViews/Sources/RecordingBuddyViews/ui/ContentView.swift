//
//  ContentView.swift
//  Recording Buddy
//
//  Created by Pedro Tacla Yamada on 27/8/21.
//

import SwiftUI

public struct ContentView: View {
    @Environment(\.appContext) var appContext: AppContext
    var engineStateViewModel: EngineStateViewModel

    public init(
        engineStateViewModel: EngineStateViewModel
    ) {
        self.engineStateViewModel = engineStateViewModel
    }

    public var body: some View {
        VStack {
            EngineStateView(model: engineStateViewModel)
            ChartControl()
            Button("Settings", action: {
                try? appContext.navigationDelegate().navigate(.openSettings)
            })
        }.frame(minWidth: 300, minHeight: 500, alignment: .topLeading)
    }
}


struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView(engineStateViewModel: EngineStateViewModel(isRunning: true))
    }
}

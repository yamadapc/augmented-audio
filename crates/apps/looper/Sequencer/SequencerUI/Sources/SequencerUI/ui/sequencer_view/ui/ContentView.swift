//
//  ContentView.swift
//  Sequencer
//
//  Created by Pedro Tacla Yamada on 28/2/2022.
//

import SwiftUI

public struct ContentView: View {
    @StateObject var store = Store()

    public init() {}

    public var body: some View {
        SequencerView()
            .background(SequencerColors.black1)
            .frame(maxWidth: .infinity, minHeight: 800, maxHeight: .infinity)
            .environmentObject(store)
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}

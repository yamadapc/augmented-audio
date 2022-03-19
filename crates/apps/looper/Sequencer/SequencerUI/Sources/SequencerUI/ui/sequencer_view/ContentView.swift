//
//  ContentView.swift
//  Sequencer
//
//  Created by Pedro Tacla Yamada on 28/2/2022.
//

import SwiftUI

public struct ContentView: View {
    public init() {}

    public var body: some View {
        SequencerView()
            .background(SequencerColors.black1)
            .frame(maxWidth: .infinity, minHeight: 850, maxHeight: .infinity)
            .frame(idealWidth: 900, idealHeight: 850)
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}

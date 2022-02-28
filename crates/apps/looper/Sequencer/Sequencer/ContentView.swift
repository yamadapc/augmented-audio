//
//  ContentView.swift
//  Sequencer
//
//  Created by Pedro Tacla Yamada on 28/2/2022.
//

import SwiftUI

struct ContentView: View {
    var body: some View {
        SequencerView()
        .background(SequencerColors.black1)
        .frame(maxWidth: .infinity, maxHeight: .infinity)
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}

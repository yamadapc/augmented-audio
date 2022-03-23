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
        let view = SequencerView()
            .background(SequencerColors.black1)
            .frame(minWidth: 900, maxWidth: .infinity, minHeight: 900, maxHeight: .infinity)
            .frame(idealWidth: 1000, idealHeight: 850)

        ZStack {
            if #available(macOS 11.0, *) {
                view.preferredColorScheme(.dark)
            } else {
                view
            }
        }
    }
}

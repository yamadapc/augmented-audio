//
//  File.swift
//
//
//  Created by Pedro Tacla Yamada on 12/3/2022.
//

import SwiftUI

struct TransportControlsView: View {
    var body: some View {
        HStack(alignment: .center) {
            Button(action: {}) {
                if #available(macOS 11.0, *) {
                    Image(systemName: "play.fill")
                } else {
                    Text("Play")
                }
            }.buttonStyle(.plain).frame(maxHeight: .infinity)
            Button(action: {}) {
                if #available(macOS 11.0, *) {
                    Image(systemName: "stop.fill")
                } else {
                    Text("Stop")
                }
            }.buttonStyle(.plain).frame(maxHeight: .infinity)
        }
        .frame(maxWidth: .infinity)
    }
}

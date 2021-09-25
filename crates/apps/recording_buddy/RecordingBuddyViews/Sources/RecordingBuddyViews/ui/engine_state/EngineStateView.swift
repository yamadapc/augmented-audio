//
//  EngineStateView.swift
//  Recording Buddy
//
//  Created by Pedro Tacla Yamada on 27/8/21.
//

import SwiftUI

public class EngineStateViewModel: ObservableObject {
    @Published var isRunning: Bool

    public init(isRunning: Bool) {
        self.isRunning = isRunning
    }
}

struct EngineStateView: View {
    @ObservedObject var model: EngineStateViewModel
    
    var body: some View {
        HStack {
            Text("Audio Engine")
                .frame(maxWidth: .infinity, alignment: .leading)
            GroupBox {
                HStack {
                    Toggle(runningText(), isOn: $model.isRunning)
                        .toggleStyle(SwitchToggleStyle())
                }
            }
        }.frame(alignment: .topLeading).padding(10.0)
    }

    func runningText() -> String {
        if model.isRunning {
            return "On"
        } else {
            return "Off"
        }
    }
}

struct EngineStateView_Previews: PreviewProvider {
    static var previews: some View {
        EngineStateView(
            model: EngineStateViewModel(isRunning: true)
        )
        EngineStateView(
            model: EngineStateViewModel(isRunning: false)
        )
    }
}

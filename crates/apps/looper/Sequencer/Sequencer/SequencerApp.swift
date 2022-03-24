//
//  SequencerApp.swift
//  Sequencer
//
//  Created by Pedro Tacla Yamada on 28/2/2022.
//

import SequencerEngine
import SequencerUI
import SwiftUI

@main
struct SequencerApp: App {
    var engineController = EngineController()
    var body: some Scene {
        WindowGroup {
            ContentView().environmentObject(engineController.store)
        }
    }
}

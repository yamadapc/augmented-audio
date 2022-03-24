//
//  ViewController.swift
//  Sequencer Mac
//
//  Created by Pedro Tacla Yamada on 9/3/2022.
//

import Cocoa
import SequencerEngine
import SequencerUI
import SwiftUI

class ViewController: NSViewController {
    var engineController = EngineController()

    override func viewDidLoad() {
        super.viewDidLoad()

        let contentView = ContentView()
            .environmentObject(engineController.store)
        let hostingView = NSHostingView(rootView: contentView)
        view = hostingView
        for constraint in hostingView.constraints {
            constraint.isActive = false
        }
        // engineController.loadExampleFileBuffer()
    }
}

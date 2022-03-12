//
//  ViewController.swift
//  Sequencer Mac
//
//  Created by Pedro Tacla Yamada on 9/3/2022.
//

import Cocoa
import SequencerUI
import SwiftUI

class ViewController: NSViewController {
    override func viewDidLoad() {
        super.viewDidLoad()

        let contentView = ContentView()
        let hostingView = NSHostingView(rootView: contentView)
        view = hostingView
    }
}

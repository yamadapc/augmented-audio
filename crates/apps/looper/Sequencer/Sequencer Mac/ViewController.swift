// = copyright ====================================================================
// Continuous: Live-looper and performance sampler
// Copyright (C) 2022  Pedro Tacla Yamada
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
// = /copyright ===================================================================
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

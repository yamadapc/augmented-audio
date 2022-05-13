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

import Cocoa
import SequencerEngine
import SequencerUI
import SwiftUI

/**
 * ViewController for the main application window.
 */
class MainWindowViewController: NSViewController {
    override func viewDidLoad() {
        super.viewDidLoad()

        let engineController: EngineController = (NSApp.delegate as! AppDelegate).engineController
        let contentView = ContentView()
            .environmentObject(engineController.store)

        let cachingView = NSHostingView(rootView: contentView)
        cachingView.translatesAutoresizingMaskIntoConstraints = true
        cachingView.autoresizingMask = [.height, .width]
        view = cachingView
    }
}

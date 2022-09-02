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
//  SequencerUI-Playground
//
//  Created by Pedro Tacla Yamada on 3/9/2022.
//

import SequencerUI
import SwiftUI
import UIKit

class ViewController: UIViewController {
    override func viewDidLoad() {
        super.viewDidLoad()

        let rootView = PlaygroundRootView()
        let hostingViewController = UIHostingController(rootView: rootView)
        addChild(hostingViewController)

        hostingViewController.view.translatesAutoresizingMaskIntoConstraints = true
        hostingViewController.view.autoresizingMask = [.flexibleWidth, .flexibleHeight]
        hostingViewController.view.frame = view.frame

        view.addSubview(hostingViewController.view)

        view.translatesAutoresizingMaskIntoConstraints = true
        view.autoresizingMask = [.flexibleWidth, .flexibleHeight]
    }
}

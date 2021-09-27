//
//  EngineController.swift
//  Recording Buddy
//
//  Created by Pedro Tacla Yamada on 27/9/21.
//

import Foundation
import RecordingBuddyViews

class EngineController {
    let engineStateViewModel: EngineStateViewModel
    let engineService = AudioEngineService()

    init(engineStateViewModel: EngineStateViewModel) {
        self.engineStateViewModel = engineStateViewModel
        self.setup()
    }

    func setup() {
        self.engineService.start()
    }
}

//
//  Context.swift
//  Recording Buddy
//
//  Created by Pedro Tacla Yamada on 31/8/21.
//

import Foundation
import Cocoa
import RecordingBuddyViews

class AppContextImpl: AppContext {
    let handler: ChartHandler

    init() {
        initializeLogger()
        self.handler = ChartHandlerImpl()
    }

    func chartHandler() -> ChartHandler {
        return handler
    }
}

class ChartHandlerImpl: ChartHandler {
    func onChartView(_ nsView: NSView) {

    }
}

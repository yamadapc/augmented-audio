//
//  Context.swift
//  Recording Buddy
//
//  Created by Pedro Tacla Yamada on 31/8/21.
//

import Foundation
import Cocoa
import RecordingBuddyViews

@available(macOS 11.0, *)
class AppContextImpl {
    let handler = ChartHandlerImpl()
    let engineStateViewModel = EngineStateViewModel(isRunning: true)
    let settingsController = SettingsController()
    let navigationDelegateImpl: NavigationDelegateImpl
    let engineController: EngineController

    init() {
        self.navigationDelegateImpl = NavigationDelegateImpl(settingsController: settingsController)
        initializeLogger()
        self.engineController = EngineController(engineStateViewModel: engineStateViewModel)
        self.engineController.setup()
    }

    func engineState() -> EngineStateViewModel {
        return engineStateViewModel
    }
}

@available(macOS 11.0, *)
extension AppContextImpl: AppContext {
    func chartHandler() -> ChartHandler {
        return handler
    }

    func navigationDelegate() -> NavigationDelegate {
        return navigationDelegateImpl
    }
}

class ChartHandlerImpl: ChartHandler {
    func onChartView(_ nsView: NSView) {
        let nsViewPtr = Unmanaged.passRetained(nsView).toOpaque()
        chart_handler_on_chart_view(nsViewPtr)
    }
}

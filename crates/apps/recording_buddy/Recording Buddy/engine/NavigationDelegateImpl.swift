//
//  NavigationDelegateImpl.swift
//  Recording Buddy
//
//  Created by Pedro Tacla Yamada on 1/9/21.
//

import Foundation
import Cocoa
import RecordingBuddyViews
import SwiftUI

@available(macOS 11.0, *)
class NavigationDelegateImpl: NavigationDelegate {
    let settingsController: SettingsController

    init(settingsController: SettingsController) {
        self.settingsController = settingsController
    }

    func navigate(_ event: NavigationEvent) {
        switch (event) {
        case .openSettings:
            self.settingsController.openSettings()
        }
    }
}

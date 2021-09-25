//
//  SettingsController.swift
//  Recording Buddy
//
//  Created by Pedro Tacla Yamada on 31/8/21.
//

import Foundation
import SwiftUI
import RecordingBuddyViews
import Combine

@available(macOS 11.0, *)
class SettingsController: NSObject, NSWindowDelegate {
    let audioOptionsService = AudioOptionsService()
    let audioOptionsModel = AudioOptionsModel()
    let availableAudioOptionsModel = AvailableAudioOptionsModel()
    var settingsWindow: NSWindow? = nil
    private var subscriptions = Set<AnyCancellable>()

    func refreshModels() {
        let availableOptions = audioOptionsService.getAvailableOptions()
        let model = self.availableAudioOptionsModel
        DispatchQueue.main.async {
            model.hostIds = availableOptions.hostIds
            model.inputIds = availableOptions.inputIds
            model.outputIds = availableOptions.outputIds
        }
    }

    func setupSubscriptions() {
        self.subscriptions.removeAll()
        audioOptionsModel.objectWillChange
            .sink(receiveValue: {
                DispatchQueue.main.async {
                    let model = self.audioOptionsModel
                    let options = AudioOptions(
                        hostId: model.hostId,
                        inputId: model.inputId
                    )
                    self.audioOptionsService.setOptions(model: options)
                }
            })
            .store(in: &subscriptions)
    }

    func windowWillClose(_ notification: Notification) {
        self.settingsWindow = nil
        self.subscriptions.removeAll()
    }

    func openSettings() {
        DispatchQueue.global(qos: .background).async {
            self.refreshModels()
            self.setupSubscriptions()
        }

        if let window = settingsWindow {
            window.makeKeyAndOrderFront(self)
            return
        }

        let contentView = AudioSettingsView(
            model: audioOptionsModel,
            audioInfo: availableAudioOptionsModel
        )
        let window = NSWindow(
            contentRect: NSRect(x: 0, y: 0, width: DEFAULT_WIDTH, height: DEFAULT_HEIGHT),
            styleMask: [.titled, .closable, .miniaturizable, .resizable, .fullSizeContentView],
            backing: .buffered,
            defer: false
        )
        window.contentView = NSHostingView(rootView: contentView)
        window.isReleasedWhenClosed = false
        window.tabbingMode = .disallowed
        window.title = "Settings"
        window.center()
        window.delegate = self
        window.makeKeyAndOrderFront(self)
        settingsWindow = window
    }
}

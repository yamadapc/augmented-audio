//
//  AudioUnitViewController.swift
//  Sequencer AudioUnit
//
//  Created by Pedro Tacla Yamada on 23/3/2022.
//

import CoreAudioKit
import SequencerUI
import SwiftUI

public class AudioUnitViewController: AUViewController, AUAudioUnitFactory {
    var audioUnit: AUAudioUnit?

    override public func viewDidLoad() {
        super.viewDidLoad()

        if audioUnit == nil {
            return
        }

        // Get the parameter tree and add observers for any parameters that the UI needs to keep in sync with the AudioUnit
        let contentView = ContentView()
            .environmentObject(Store(engine: nil))
        let hostingView = NSHostingView(rootView: contentView)
        view = hostingView
        for constraint in hostingView.constraints {
            constraint.isActive = false
        }
    }

    public func createAudioUnit(with componentDescription: AudioComponentDescription) throws -> AUAudioUnit {
        audioUnit = try Sequencer_AudioUnitAudioUnit(componentDescription: componentDescription, options: [])

        return audioUnit!
    }
}

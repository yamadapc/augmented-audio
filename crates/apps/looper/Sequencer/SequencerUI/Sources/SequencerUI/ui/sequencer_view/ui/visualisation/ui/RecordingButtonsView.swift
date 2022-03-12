//
//  File.swift
//
//
//  Created by Pedro Tacla Yamada on 12/3/2022.
//

import SwiftUI

struct RecordingButtonsView: View {
    var store: Store
    @ObservedObject var looperState: LooperState

    var body: some View {
        VStack {
            TrackButton(
                action: { store.onClickRecord() },
                label: "Record",
                isSelected: looperState.isRecording,
                backgroundColor: looperState.isRecording ? SequencerColors.red : nil
            )
            TrackButton(
                action: { store.onClickPlay() },
                label: "Play",
                isDisabled: looperState.isEmpty,
                isSelected: false,
                backgroundColor: looperState.isPlaying ? SequencerColors.green : nil
            )
            TrackButton(
                action: { store.onClickClear() },
                label: "Clear",
                isDisabled: looperState.isEmpty,
                isSelected: false
            )
        }
    }
}

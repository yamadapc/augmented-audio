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
import SwiftUI

struct RecordButtonView: View {
    @ObservedObject var store: Store
    @ObservedObject var trackState: TrackState
    @State var isAnimating = false
    @State var level: Double = 0

    // TODO: This is broken
    // let timer = Timer.publish(every: 1 / 60, on: .current, in: .common).autoconnect()

    var body: some View {
        let button = ContinuousButton(
            action: { store.onClickRecord() },
                label: "Record",
                isDisabled: store.midiMappingActive,
                isSelected: trackState.looperState.isRecording,
                backgroundColor: buttonColor().opacity(min(max(level * 20, 0.3), 1))
        )
                .bindToParameterId(
                        store: store,
                        parameterId: .recordButton(trackId: trackState.id)
                )
                .testId("recordButton")
        // .onReceive(self.timer) { _ in
        //     self.level = Double(store.engine?.getInputLevel() ?? 0)
        // }

        if trackState.looperState == .recordingScheduled {
            button
                    .onAppear {
                        withAnimation(.easeInOut(duration: BUTTON_ANIMATION_DURATION_SECS).repeatForever()) {
                            isAnimating.toggle()
                        }
                    }
        } else {
            button
        }
    }

    func buttonColor() -> Color {
        if trackState.looperState.isRecording || trackState.looperState == .playingScheduled {
            return SequencerColors.recordColor
        }

        if trackState.looperState == .recordingScheduled {
            return isAnimating ? SequencerColors.recordColor : SequencerColors.black
        }

        return SequencerColors.black
    }
}

struct RecordButtonView_Preview: PreviewProvider {
    static var previews: some View {
        Group {
            RecordButtonView(
                store: Store(engine: nil),
                trackState: TrackState(id: 0)
            )
            .previewDisplayName("Normal")

            let recordingStore = Store.recording()
            RecordButtonView(
                store: recordingStore,
                trackState: recordingStore.currentTrackState()
            )
            .previewDisplayName("Recording")

            let recordingScheduledStore = Store.recordingScheduled()
            RecordButtonView(
                store: recordingScheduledStore,
                trackState: recordingScheduledStore.currentTrackState()
            )
            .previewDisplayName("Recording Scheduled")
            .onAppear {
                DispatchQueue.main.asyncAfter(
                    deadline: .now().advanced(by: .seconds(2))
                ) {
                    print("YO")
                    recordingScheduledStore.currentTrackState().looperState = .recording
                }
            }
        }
    }
}

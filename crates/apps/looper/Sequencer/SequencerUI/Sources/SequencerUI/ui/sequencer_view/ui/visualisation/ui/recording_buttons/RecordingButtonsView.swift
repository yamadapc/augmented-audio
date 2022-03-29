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

let BUTTON_ANIMATION_DURATION_SECS = 0.6

struct RecordingButtonsView: View {
    @ObservedObject var store: Store
    @ObservedObject var trackState: TrackState

    var body: some View {
        VStack {
            RecordButtonView(store: store, trackState: trackState)
            PlayButtonView(store: store, trackState: trackState)

            TrackButton(
                action: { store.onClickClear() },
                label: "Clear",
                isDisabled: trackState.looperState.isEmpty || store.midiMappingActive,
                isSelected: false
            )
            .bindToParameterId(store: store, parameterId: .clearButton(trackId: trackState.id))
        }
    }
}

struct RecordingButtonsView_Preview: PreviewProvider {
    static var previews: some View {
        Group {
            let store = Store(engine: nil)
            RecordingButtonsView(
                store: store,
                trackState: store.currentTrackState()
            )
            .previewDisplayName("Normal")

            let storeRecording = Store.recording()
            RecordingButtonsView(
                store: storeRecording,
                trackState: storeRecording.currentTrackState()
            )
            .previewDisplayName("Recording")

            let storePlaying = Store.playing()
            RecordingButtonsView(
                store: storePlaying,
                trackState: storePlaying.currentTrackState()
            )
            .previewDisplayName("Playing")

            let storePlayingScheduled = Store.playingScheduled()
            RecordingButtonsView(
                store: storePlayingScheduled,
                trackState: storePlayingScheduled.currentTrackState()
            )
            .previewDisplayName("Playing scheduled")
        }
    }
}

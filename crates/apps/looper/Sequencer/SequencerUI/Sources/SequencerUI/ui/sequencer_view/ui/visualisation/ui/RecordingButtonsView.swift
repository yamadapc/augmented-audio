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

struct RecordingButtonsView: View {
    var store: Store
    @ObservedObject var trackState: TrackState

    var body: some View {
        VStack {
            TrackButton(
                action: { store.onClickRecord() },
                label: "Record",
                isSelected: trackState.looperState.isRecording,
                backgroundColor: trackState.looperState.isRecording ? SequencerColors.red : nil
            )
            .bindToParameterId(store: store, parameterId: .recordButton(trackId: trackState.id))

            TrackButton(
                action: { store.onClickPlay() },
                label: "Play",
                isDisabled: trackState.looperState.isEmpty,
                isSelected: false,
                backgroundColor: trackState.looperState.isPlaying ? SequencerColors.green : nil
            )
            .bindToParameterId(store: store, parameterId: .playButton(trackId: trackState.id))

            TrackButton(
                action: { store.onClickClear() },
                label: "Clear",
                isDisabled: trackState.looperState.isEmpty,
                isSelected: false
            )
            .bindToParameterId(store: store, parameterId: .clearButton(trackId: trackState.id))
        }
    }
}

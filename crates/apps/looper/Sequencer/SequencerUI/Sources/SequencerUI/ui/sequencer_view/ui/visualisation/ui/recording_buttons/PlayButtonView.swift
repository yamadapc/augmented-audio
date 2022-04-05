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

struct PlayButtonView: View {
    @ObservedObject var store: Store
    @ObservedObject var trackState: TrackState
    @State var isAnimating = false

    var body: some View {
        let view = ContinuousButton(
                action: { store.onClickPlay() },
                label: "Play",
                isDisabled: trackState.looperState.isEmpty || store.midiMappingActive,
                isSelected: false,
                backgroundColor: buttonColor()
        )
                .bindToParameterId(store: store, parameterId: .playButton(trackId: trackState.id))

        if trackState.looperState == .playingScheduled {
            view
                .onAppear {
                    withAnimation(.easeInOut(duration: BUTTON_ANIMATION_DURATION_SECS).repeatForever()) {
                        isAnimating.toggle()
                    }
                }
        } else {
            view
        }
    }

    func buttonColor() -> Color? {
        if trackState.looperState.isPlaying {
            return SequencerColors.green
        }

        if trackState.looperState == .playingScheduled {
            return isAnimating ? SequencerColors.green : nil
        }

        return SequencerColors.black
    }
}

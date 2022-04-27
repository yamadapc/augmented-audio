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

struct MixKnobView: View {
    var trackId: Int

    @EnvironmentObject var store: Store
    @ObservedObject var trackState: TrackState

    @State var value: Float = 0

    var body: some View {
        ZStack {
            Text("\(trackState.volumeParameter.label)")
                .foregroundColor(SequencerColors.white.opacity(0.4))
                .rotationEffect(Angle(degrees: -90))
                .transformEffect(.init(translationX: 15, y: 0))

            KnobSliderView(
                value: $trackState.volumeParameter.value,
                defaultValue: trackState.volumeParameter.defaultValue,
                style: .vertical,
                tickColor: SequencerColors.black3,
                railColor: SequencerColors.black3,
                handleColor: SequencerColors.black0
            )
            .bindToParameterId(
                store: store,
                parameterId: trackState.volumeParameter.globalId
            )
            // ParameterKnobView(parameter: trackState.volumeParameter)
        }
    }
}

struct MixPanelContentView: View {
    @EnvironmentObject var store: Store

    var body: some View {
        HStack {
            ForEach(0 ..< 8) { i in
                HStack(spacing: 0) {
                    Rectangle()
                        .fill(SequencerColors.black)
                        .frame(width: 1)
                        .frame(maxHeight: .infinity)

                    MixKnobView(trackId: i, trackState: store.trackStates[i])
                        .frame(width: 50)

                    Rectangle()
                        .fill(SequencerColors.black)
                        .frame(width: 1)
                        .frame(maxHeight: .infinity)
                }
            }

            ParameterKnobView(parameter: store.metronomeVolume)
        }
    }
}

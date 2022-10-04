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

struct LoopVisualisationView: View {
    @EnvironmentObject var store: Store
    @ObservedObject var trackState: TrackState
    @State var tick: Int = 0

    var body: some View {
        if #available(macOS 12.0, *) {
            self.renderInner(tick: 0)
            // TimelineView(.periodic(from: .now, by: 1 / 30)) { timeline in
            //     self.renderInner(tick: Int(timeline.date.timeIntervalSince1970 * 1000))
            // }
        } else {
            self.renderInner(tick: 0)
        }
    }

    func renderInner(tick: Int) -> some View {
        VStack {
            GeometryReader { geometry in
                AudioPathMetalView(layer: trackState.metalLayer, size: geometry.size)
                    .frame(maxWidth: .infinity, maxHeight: .infinity)
                    .onAppear {
                        store.engine?.startRendering(looperId: trackState.id, layer: trackState.metalLayer!)
                    }
            }

//            if let buffer = trackState.buffer {
//                GeometryReader { geometry in
//                    ZStack(alignment: .topLeading) {
//                        AudioPathView(tick: tick, buffer: buffer, geometry: geometry)
//                            .equatable()
//                        PlayheadView(position: trackState.position, size: geometry.size)
//                        SourceParametersOverlayView(sourceParameters: trackState.sourceParameters)
//                    }
//                }
//            } else {
//                Text("No loop buffer")
//                    .frame(maxHeight: .infinity)
//            }

            HStack {
                ToggleParameterView(parameter: trackState.sourceParameters.loopEnabled)
                EnumParameterView(parameter: trackState.quantizationParameters.quantizationMode)
                EnumParameterView(parameter: trackState.quantizationParameters.tempoControlMode)
            }
            .frame(maxWidth: .infinity, alignment: .leading)
        }
        .padding(PADDING)
    }
}

struct LoopVisualisationView_Previews: PreviewProvider {
    static var previews: some View {
        LoopVisualisationView(trackState: TrackState(id: 0))
            .cornerRadius(BORDER_RADIUS)
    }
}

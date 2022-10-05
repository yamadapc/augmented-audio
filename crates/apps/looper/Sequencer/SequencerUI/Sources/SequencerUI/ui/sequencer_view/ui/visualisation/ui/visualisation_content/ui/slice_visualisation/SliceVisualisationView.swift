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

struct SliceVisualisationView: View {
    @EnvironmentObject var store: Store
    @ObservedObject var trackState: TrackState
    var timer = Timer.publish(every: 1 / 60, on: .main, in: .default).autoconnect()

    var body: some View {
        VStack {
            if let buffer = trackState.buffer {
                GeometryReader { geometry in
                    ZStack(alignment: .topLeading) {
                        AudioPathMetalView(layer: trackState.metalLayer, size: geometry.size)
                                .frame(maxWidth: .infinity, maxHeight: .infinity)
                                .onAppear {
                                    store.engine?.startRendering(looperId: trackState.id)
                                }
                                .onReceive(timer) { _ in
                                    store.engine?.startRendering(looperId: trackState.id)
                                }

                        if let sliceBuffer = trackState.sliceBuffer {
                            ForEach(0..<sliceBuffer.count, id: \.self) { i in
                                let positionSamples = sliceBuffer[i]
                                let offsetPerc = CGFloat(positionSamples) / CGFloat(buffer.count)

                                GeometryReader { geometry in
                                    Rectangle()
                                            .fill()
                                            .frame(width: 1)
                                        .frame(maxHeight: .infinity)
                                        .position(x: 0, y: 0)
                                        .offset(x: offsetPerc * geometry.size.width, y: geometry.size.height / 2)
                                }
                            }
                        }
                    }
                }
            } else {
                Text("No loop buffer")
                    .frame(maxHeight: .infinity)
            }

            HStack {
                ToggleParameterView(parameter: trackState.sourceParameters.sliceEnabled)
            }
            .frame(maxWidth: .infinity, alignment: .leading)
        }
        .padding(PADDING)
    }
}

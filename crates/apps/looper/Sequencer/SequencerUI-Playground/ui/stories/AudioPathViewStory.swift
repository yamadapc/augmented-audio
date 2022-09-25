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
import SequencerUI
import SequencerEngine
import Logging

struct AudioPathViewStory: View {
    @State
    var buffer: UnsafeBufferTrackBuffer? = nil
    @State
    var renderStartTime: DispatchTime? = nil
    let logger = Logger(label: "AudioPathViewStory")

    var body: some View {
        ZStack {
            if let buffer = buffer {
                GeometryReader { geometry in
                    AudioPathView(tick: 0, buffer: buffer, geometry: geometry)
                        .environmentObject(Store(engine: nil))
                }
                .onAppear {
                    guard let renderStartTime = renderStartTime else {
                        return
                    }
                    let renderingTime = (DispatchTime.now().uptimeNanoseconds - renderStartTime.uptimeNanoseconds) / 1_000_000
                    logger.info("Time to render file", metadata: [
                        "timeMs": .string(String(describing: renderingTime)),
                    ])
                }
            } else {
                Text("Loading audio file...")
            }
        }.onAppear {
            let startTime = DispatchTime.now()
            DispatchQueue.global(qos: .userInitiated).async {
                let buffer = getSampleBuffer()
                let loadingTime = (DispatchTime.now().uptimeNanoseconds - startTime.uptimeNanoseconds) / 1_000_000
                logger.info("Time to load file", metadata: [
                    "timeMs": .string(String(describing: loadingTime)),
                ])

                self.renderStartTime = DispatchTime.now()
                self.buffer = buffer
            }
        }
    }
}

fileprivate func getSampleBuffer() -> UnsafeBufferTrackBuffer {
    return UnsafeBufferTrackBuffer(inner: EngineImpl.getExampleBuffer())
}

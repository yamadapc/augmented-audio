import SwiftUI

struct SliceVisualisationView: View {
    @ObservedObject var trackState: TrackState

    var body: some View {
        VStack {
            ZStack {
                if let buffer = trackState.buffer {
                    GeometryReader { geometry in
                        ZStack(alignment: .topLeading) {
                            AudioPathView(
                                tick: 0,
                                buffer: buffer,
                                geometry: geometry
                            )
                            .equatable()

                            PlayheadView(position: trackState.position, size: geometry.size)

                            if let sliceBuffer = trackState.sliceBuffer {
                                ForEach(0 ..< sliceBuffer.count, id: \.self) { i in
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
                    .padding()
                } else {
                    Text("No loop buffer")
                }
            }
            .frame(maxHeight: .infinity)

            HStack {
                Text("Auto slice enabled")
            }
            .frame(maxWidth: .infinity, alignment: .leading)
            .padding(PADDING)
        }
    }
}

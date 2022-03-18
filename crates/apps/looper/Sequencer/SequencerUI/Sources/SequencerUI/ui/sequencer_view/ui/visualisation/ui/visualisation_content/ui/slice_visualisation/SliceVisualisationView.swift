import SwiftUI

struct SliceVisualisationView: View {
    @ObservedObject var trackState: TrackState

    var body: some View {
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
                    }
                }
                .padding()
            } else {
                Text("No loop buffer")
            }
        }
    }
}

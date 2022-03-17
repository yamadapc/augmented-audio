import SwiftUI

struct LoopVisualisationView: View {
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
        ZStack {
            if let buffer = trackState.buffer {
                GeometryReader { geometry in
                    ZStack(alignment: .topLeading) {
                        AudioPathView(tick: tick, buffer: buffer, geometry: geometry)
                            .equatable()
                        PlayheadView(trackState: trackState, size: geometry.size)
                        SourceParametersOverlayView(sourceParameters: trackState.sourceParameters)
                    }
                }
                .padding()
            } else {
                Text("No loop buffer")
            }
        }
    }
}

struct LoopVisualisationView_Previews: PreviewProvider {
    static var previews: some View {
        LoopVisualisationView(trackState: TrackState(id: 0))
            .cornerRadius(BORDER_RADIUS)
    }
}

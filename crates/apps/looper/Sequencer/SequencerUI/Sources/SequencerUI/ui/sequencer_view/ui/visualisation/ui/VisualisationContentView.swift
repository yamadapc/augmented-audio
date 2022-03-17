import SwiftUI

struct VisualisationContentView: View {
    @EnvironmentObject var store: Store

    var body: some View {
        let currentTrack = store.currentTrackState()
        switch store.selectedTab {
        case .source:
            LoopVisualisationView(trackState: currentTrack)
        case .lfos:
            HStack {
                LFOVisualisationView(model: currentTrack.lfo1)
                Rectangle()
                    .fill(SequencerColors.black3)
                    .frame(width: 1.0)
                    .frame(maxHeight: .infinity)
                LFOVisualisationView(model: currentTrack.lfo2)
            }
        case .envelope:
            EnvelopeVisualisationView(model: currentTrack.envelope)
        default:
            Text("No tab content").foregroundColor(SequencerColors.white)
        }
    }
}

import SwiftUI

struct VisualisationContentView: View {
    @EnvironmentObject var store: Store

    var body: some View {
        let currentTrack = store.currentTrackState()
        switch store.selectedTab {
        case .source:
            LoopVisualisationView(trackState: currentTrack)
        case .slice:
            SliceVisualisationView(trackState: currentTrack)
        case .fx:
            EffectsRowView()
        case .envelope:
            EnvelopeVisualisationView(model: currentTrack.envelope)
        default:
            Text("No tab content").foregroundColor(SequencerColors.white)
        }
    }
}

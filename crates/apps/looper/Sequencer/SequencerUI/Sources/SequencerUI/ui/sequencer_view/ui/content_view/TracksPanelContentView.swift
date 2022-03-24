import SwiftUI

struct TracksPanelContentView: View {
    @EnvironmentObject var store: Store

    var body: some View {
        HStack {
            let tracksPanelContentView = HStack(alignment: .top, spacing: 30) {
                switch store.selectedTab {
                case .mix:
                    MixPanelContentView()
                case .source, .slice:
                    SourcePanelContentView(
                        sourceParameters: store.currentTrackState().sourceParameters
                    )

                case .envelope:
                    EnvelopePanelContentView(
                        envelope: store.currentTrackState().envelope
                    )
                case .fx:
                    EffectsPanelContentView()
                case .lfos:
                    HStack(spacing: PADDING) {
                        LFOPanelContentView(lfoState: store.currentTrackState().lfo1)
                        LFOPanelContentView(lfoState: store.currentTrackState().lfo2)
                    }
                }
            }
            .padding(PADDING * 2)
            .frame(maxWidth: .infinity, maxHeight: .infinity)

            tracksPanelContentView
        }.frame(maxHeight: .infinity)
            .foregroundColor(SequencerColors.white)
            .background(SequencerColors.black)
    }
}

struct TracksPanelContentView_Previews: PreviewProvider {
    static var previews: some View {
        TracksPanelContentView().environmentObject(Store(engine: nil))
    }
}

import OSCKit
import SwiftUI

struct VisualisationView: View {
    @EnvironmentObject var store: Store

    var body: some View {
        HStack {
            RecordingButtonsView(
                store: store,
                trackState: store.currentTrackState()
            )
            ZStack {
                Rectangle()
                    .fill(SequencerColors.black1)
                    .frame(maxWidth: .infinity, maxHeight: .infinity)
                Rectangle()
                    .fill(SequencerColors.black)
                    .cornerRadius(BORDER_RADIUS)
                    .frame(maxWidth: .infinity, maxHeight: .infinity)

                VisualisationContentView()
                    .foregroundColor(SequencerColors.white)
            }
        }
        .padding(EdgeInsets(top: 0, leading: PADDING, bottom: PADDING, trailing: PADDING))
        .frame(maxHeight: 260)
    }
}

struct VisualisationView_Previews: PreviewProvider {
    static var previews: some View {
        VisualisationView().environmentObject(Store(engine: nil))
    }
}

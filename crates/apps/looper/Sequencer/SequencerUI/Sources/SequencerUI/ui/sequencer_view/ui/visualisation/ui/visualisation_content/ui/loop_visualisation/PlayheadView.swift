import SwiftUI

struct PlayheadView: View {
    @ObservedObject var trackState: TrackState
    var size: CGSize

    var body: some View {
        Rectangle()
            .fill(SequencerColors.green)
            .frame(width: 1.0, height: size.height)
            // y is set to an arbitrary nº
            .position(x: 0.0, y: 110)
            .transformEffect(
                .init(translationX: size.width * CGFloat(trackState.positionPercent), y: 0.0))
    }
}

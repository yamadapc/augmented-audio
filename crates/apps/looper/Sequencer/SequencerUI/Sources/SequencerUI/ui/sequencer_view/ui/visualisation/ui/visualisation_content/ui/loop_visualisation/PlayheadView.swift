import SwiftUI

struct PlayheadView: View {
    @ObservedObject var position: LoopPosition
    var size: CGSize

    var body: some View {
        GeometryReader { geometry in
            Rectangle()
                .fill(SequencerColors.green)
                .frame(width: 1.0, height: size.height)
                .position(x: 0.0, y: geometry.size.height / 2)
                .transformEffect(
                    .init(translationX: size.width * CGFloat(position.positionPercent), y: 0.0))
        }
    }
}

import SwiftUI

struct ParameterLockAnimationView: View {
    @ObservedObject var focusState: FocusState
    var parameterId: ObjectId
    @State var isAnimating = false

    var body: some View {
        if focusState.draggingStep != nil && focusState.mouseOverObject == parameterId {
            ZStack {
                Circle()
                    .stroke(SequencerColors.green, lineWidth: 2)
                    .frame(width: 30 * 2, height: 30 * 2)
                    .opacity(self.isAnimating ? 0.0 : 1.0)
                    .scaleEffect(self.isAnimating ? 1.5 : 0.0, anchor: .center)
                    .animation(
                        .linear(duration: 0.75).repeatForever(autoreverses: false),
                        value: self.isAnimating
                    )
                Circle()
                    .stroke(SequencerColors.green, lineWidth: 2.0)
                    .frame(width: 20 * 2, height: 20 * 2)
                    .opacity(self.isAnimating ? 0.0 : 1.0)
                    .scaleEffect(self.isAnimating ? 1.5 : 0.0, anchor: .center)
                    .animation(
                        .linear(duration: 0.75).repeatForever(autoreverses: false),
                        value: self.isAnimating
                    )
                Circle()
                    .stroke(SequencerColors.green, lineWidth: 2)
                    .frame(width: 10 * 2, height: 10 * 2)
                    .opacity(self.isAnimating ? 0.0 : 1.0)
                    .scaleEffect(self.isAnimating ? 1.5 : 0.0, anchor: .center)
                    .animation(
                        .linear(duration: 0.75).repeatForever(autoreverses: false),
                        value: self.isAnimating
                    )
            }
            .animation(
                .linear(duration: 0.75).repeatForever(autoreverses: false),
                value: self.isAnimating
            )
            .onAppear {
                self.isAnimating = true
            }
            .onDisappear {
                self.isAnimating = false
            }
        }
    }
}

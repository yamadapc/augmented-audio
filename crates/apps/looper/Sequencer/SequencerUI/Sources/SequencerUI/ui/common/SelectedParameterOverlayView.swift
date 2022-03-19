import SwiftUI

struct SelectedParameterOverlayView: View {
    @ObservedObject var focusState: FocusState
    var parameterId: ObjectId
    var showSelectionOverlay: Bool

    var body: some View {
        ZStack {
            if showSelectionOverlay && focusState.selectedObject == parameterId {
                Rectangle()
                    .stroke(SequencerColors.white.opacity(0.3))
                    .scaleEffect(1.2)
            }
        }
        .allowsHitTesting(false)
    }
}

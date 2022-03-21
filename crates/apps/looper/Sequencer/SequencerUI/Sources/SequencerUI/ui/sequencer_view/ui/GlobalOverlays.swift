import SwiftUI

struct SceneDragRect: View {
    @EnvironmentObject var _store: Store
    var dragState: SceneDragState
    var body: some View {
        Rectangle()
            .fill(SequencerColors.green)
            .frame(width: 30, height: 30)
    }
}

struct SceneDragOverlayView: View {
    @ObservedObject var focusState: FocusState

    var body: some View {
        ZStack {
            if let dragState = focusState.sceneDragState {
                SceneDragRect(dragState: dragState)
                    .position(dragState.position)
                    .opacity(0.7)
            }
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
        .allowsHitTesting(false)
    }
}

struct GlobalOverlays: View {
    @EnvironmentObject var store: Store

    var body: some View {
        ZStack {
            SceneDragOverlayView(focusState: store.focusState)
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
        .allowsHitTesting(false)
    }
}

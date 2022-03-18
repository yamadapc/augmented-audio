import SwiftUI

struct StatusBarViewInner: View {
    @ObservedObject var focusState: FocusState

    var body: some View {
        if let focusedObject = focusState.mouseOverObject {
            Text(String(describing: focusedObject))
        } else {
            Text("...")
        }
    }
}

struct StatusBarView: View {
    @EnvironmentObject var store: Store

    var body: some View {
        HStack {
            StatusBarViewInner(focusState: store.focusState)
        }
        .frame(maxWidth: .infinity, alignment: .leading)
        .padding(PADDING * 0.5)
        .background(SequencerColors.black)
    }
}

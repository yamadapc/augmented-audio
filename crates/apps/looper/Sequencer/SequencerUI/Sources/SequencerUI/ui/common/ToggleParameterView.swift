import SwiftUI

struct ToggleParameterView: View {
    @ObservedObject var parameter: BooleanParameter
    @EnvironmentObject var store: Store

    var body: some View {
        Button(
            parameter.label,
            action: {
                parameter.value.toggle()
            }
        )
        .buttonStyle(.plain)
        .padding(PADDING)
        .border(
            SequencerColors.blue,
            width: 1.0
        )
        .background(
            SequencerColors.blue.opacity(
                parameter.value ? 0.8 : 0
            )
        )
        .bindToParameterId(store: store, parameterId: parameter.id, showSelectionOverlay: false)
    }
}

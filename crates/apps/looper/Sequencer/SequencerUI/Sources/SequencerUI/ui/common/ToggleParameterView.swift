import SwiftUI

struct ToggleParameterView: View {
    @ObservedObject var parameter: BooleanParameter

    var body: some View {
        Button(parameter.label, action: {
            parameter.value.toggle()
        }).buttonStyle(.plain)
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
    }
}

import SwiftUI

struct TrackButton: View {
    var action: () -> Void
    var label: String
    var isDisabled: Bool = false
    var isSelected: Bool
    var backgroundColor: Color?

    var body: some View {
        Button(
            action: action,
            label: { Text(label)
                .frame(width: 80.0, height: 80.0, alignment: .center)
                .contentShape(Rectangle())
                .foregroundColor(SequencerColors.white)
                .background(
                    RoundedRectangle(cornerRadius: BORDER_RADIUS)
                        .stroke(
                            isSelected ? SequencerColors.red : SequencerColors.black3,
                            lineWidth: 1.0
                        )
                        .background(self.backgroundColor ?? SequencerColors.black)
                )
                .cornerRadius(BORDER_RADIUS)
            }
        )
        .buttonStyle(.plain)
        .disabled(isDisabled)
    }
}

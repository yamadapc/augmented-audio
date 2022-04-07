// = copyright ====================================================================
// Continuous: Live-looper and performance sampler
// Copyright (C) 2022  Pedro Tacla Yamada
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
// = /copyright ===================================================================
import SwiftUI

struct ContinuousButton: View {
    var action: () -> Void
    var label: String
    var isDisabled: Bool = false
    var isSelected: Bool
    var backgroundColor: Color?

    var body: some View {
        Button(
            action: action,
            label: {
                Text(label)
                    .frame(width: 70.0, height: 70.0, alignment: .center)
                    .contentShape(Rectangle())
                    .foregroundColor(SequencerColors.white)
                    .background(
                        RoundedRectangle(cornerRadius: BORDER_RADIUS)
                            .stroke(
                                isSelected ? SequencerColors.selectionColorHigh : SequencerColors.black3.opacity(0.1),
                                lineWidth: 1.0
                            )
                            .background(self.backgroundColor ?? SequencerColors.black)
                    )
                    .cornerRadius(BORDER_RADIUS)
                    .shadow(color: Color.black.opacity(0.5), radius: 5, x: 0, y: 5)
            }
        )
        .buttonStyle(.plain)
        .disabled(isDisabled)
    }
}

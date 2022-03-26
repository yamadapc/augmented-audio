import Combine
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

class SelectedParameterOverlayViewModel: ObservableObject {
    @Published var isSelected: Bool
    private var cancellables = Set<AnyCancellable>()

    init(
        parameterId: ParameterId,
        focusState: FocusState
    ) {
        isSelected = focusState.selectedObject == parameterId

        focusState.$selectedObject.sink(receiveValue: { selectedObject in
            let newValue = selectedObject == parameterId
            if newValue != self.isSelected {
                self.isSelected = newValue
            }
        }).store(in: &cancellables)
    }
}

struct SelectedParameterOverlayViewInner: View {
    @ObservedObject var model: SelectedParameterOverlayViewModel

    var body: some View {
        ZStack {
            if model.isSelected {
                Rectangle()
                    .stroke(SequencerColors.white.opacity(0.3))
                    .scaleEffect(1.2)
            }
        }
        .allowsHitTesting(false)
    }
}

struct SelectedParameterOverlayView: View {
    var focusState: FocusState
    var parameterId: ParameterId
    var showSelectionOverlay: Bool

    var body: some View {
        if showSelectionOverlay {
            SelectedParameterOverlayViewInner(
                model: SelectedParameterOverlayViewModel(
                    parameterId: parameterId,
                    focusState: focusState
                )
            )
        }
    }
}

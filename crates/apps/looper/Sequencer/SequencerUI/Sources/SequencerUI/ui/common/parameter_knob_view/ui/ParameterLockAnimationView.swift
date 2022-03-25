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

class ParameterLockAnimationViewModel: ObservableObject {
    @Published var isDragingIntoThis: Bool

    var parameterId: ParameterId
    var focusState: FocusState
    var cancellables = Set<AnyCancellable>()

    init(parameterId: ParameterId, focusState: FocusState) {
        self.parameterId = parameterId
        self.focusState = focusState
        isDragingIntoThis = Self.readState(
            parameterId: parameterId,
            focusState: focusState
        )
        focusState.objectWillChange.sink(receiveValue: {
            let newValue = Self.readState(
                parameterId: self.parameterId,
                focusState: self.focusState
            )
            if newValue != self.isDragingIntoThis {
                self.isDragingIntoThis = newValue
            }
        }).store(in: &cancellables)
    }

    static func readState(parameterId: ParameterId, focusState: FocusState) -> Bool {
        return focusState.draggingSource != nil && focusState.mouseOverObject == parameterId
    }
}

struct ParameterLockAnimationViewInner: View {
    @ObservedObject var model: ParameterLockAnimationViewModel
    @State var isAnimating = false

    var body: some View {
        if model.isDragingIntoThis {
            ZStack {
                Circle()
                    .stroke(SequencerColors.green, lineWidth: 2)
                    .frame(width: 30 * 2, height: 30 * 2)
                    .opacity(self.isAnimating ? 0.0 : 1.0)
                    .scaleEffect(self.isAnimating ? 1.5 : 0.1, anchor: .center)
                    .animation(
                        .linear(duration: 0.75).repeatForever(autoreverses: false),
                        value: self.isAnimating
                    )
                Circle()
                    .stroke(SequencerColors.green, lineWidth: 2.0)
                    .frame(width: 20 * 2, height: 20 * 2)
                    .opacity(self.isAnimating ? 0.0 : 1.0)
                    .scaleEffect(self.isAnimating ? 1.5 : 0.1, anchor: .center)
                    .animation(
                        .linear(duration: 0.75).repeatForever(autoreverses: false),
                        value: self.isAnimating
                    )
                Circle()
                    .stroke(SequencerColors.green, lineWidth: 2)
                    .frame(width: 10 * 2, height: 10 * 2)
                    .opacity(self.isAnimating ? 0.0 : 1.0)
                    .scaleEffect(self.isAnimating ? 1.5 : 0.1, anchor: .center)
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

struct ParameterLockAnimationView: View {
    var focusState: FocusState
    var parameterId: ParameterId

    var body: some View {
        ParameterLockAnimationViewInner(
            model: ParameterLockAnimationViewModel(
                parameterId: parameterId,
                focusState: focusState
            )
        )
    }
}

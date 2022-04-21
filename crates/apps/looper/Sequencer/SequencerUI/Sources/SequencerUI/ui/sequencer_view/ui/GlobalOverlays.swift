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

struct SceneDragRect: View {
    @EnvironmentObject var _store: Store
    var dragState: SceneDragState
    var body: some View {
        ContinuousButton(
            action: {},
            label: scenes[dragState.scene],
            isSelected: false
        )
    }
}

struct SceneDragOverlayView: View {
    @ObservedObject var focusState: FocusState

    var body: some View {
        ZStack(alignment: .topLeading) {
            if let dragState = focusState.sceneDragState {
                Path { path in
                    path.move(to: dragState.startPosition)
                    path.addLine(to: dragState.position)
                }
                .stroke(SequencerColors.green, lineWidth: 2)
                .offset(x: 0, y: -30)
            }
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
        .coordinateSpace(name: "GlobalOverlays")
        .allowsHitTesting(false)
    }
}

@available(macOS 12.0, *)
struct ModalOverlayView: View {
    @ObservedObject var modalState: ModalState
    @EnvironmentObject var store: Store
    @State var alertShown: Bool = false

    var body: some View {
        ZStack {
            if modalState.modal == .analyticsModal {
                Rectangle()
                    .fill(SequencerColors.black0.opacity(0.8))
                    .frame(maxWidth: .infinity, maxHeight: .infinity)
                    .onAppear {
                      alertShown = true
                    }
                    .alert(
                      "Send usage statistics (you can change this in preferences later)",
                      isPresented: $alertShown
                    ) {
                        Button("Yes", action: {
                            alertShown = false
                            modalState.modal = nil
                            store.isAnalyticsEnabled = true
                        })
                        Button("No", action: {
                            alertShown = false
                            modalState.modal = nil
                            store.isAnalyticsEnabled = false
                        })
                    }
            }
        }
        .allowsHitTesting(true)
    }
}

struct GlobalOverlays: View {
    @EnvironmentObject var store: Store

    var body: some View {
        ZStack {
            SceneDragOverlayView(focusState: store.focusState)
            if #available(macOS 12.0, *) {
                ModalOverlayView(modalState: store.modalState)
            }
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
        .allowsHitTesting(false)
        .drawingGroup()
    }
}

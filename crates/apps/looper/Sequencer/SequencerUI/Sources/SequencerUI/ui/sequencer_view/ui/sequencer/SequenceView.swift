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
//
//  SwiftUIView.swift
//
//
//  Created by Pedro Tacla Yamada on 11/3/2022.
//

import Combine
import SwiftUI

struct SequenceView: View {
    struct DragState {
        let step: Int
        let position: CGPoint
        let mode: DragMode
    }

    @EnvironmentObject var store: Store
    @State var dragState: DragState?

    var body: some View {
        ZStack {
            HStack {
                ForEach(0 ..< 16) { i in
                    ConnectedStepButtonView(
                        index: i,
                        store: store,
                        stepModel: StepButtonViewModel(
                            store: store,
                            track: store.currentTrackState(),
                            index: i
                        )
                    )
                    .bindToParameterId(
                        store: store,
                        parameterId: .stepButton(trackId: store.selectedTrack, stepId: i),
                        showSelectionOverlay: false
                    )
                    .highPriorityGesture(
                        DragGesture(coordinateSpace: .named("SequenceViewZStack"))
                            .onChanged { drag in startDrag(i, drag, .lock) }
                            .onEnded { _ in endDrag() }
                    )
                    #if os(macOS)
                    .highPriorityGesture(
                        DragGesture(coordinateSpace: .named("SequenceViewZStack"))
                            .modifiers(.option)
                            .onChanged { drag in startDrag(i, drag, .copy) }
                            .onEnded { _ in endDrag() }
                    )
                    #endif
                }
            }
            .padding(PADDING)
            .background(SequencerColors.black1)
            .frame(maxWidth: .infinity)

            if let dragState = self.dragState {
                ZStack {
                    ConnectedStepButtonView(
                        index: dragState.step,
                        store: store,
                        stepModel: StepButtonViewModel(
                            store: store,
                            track: store.currentTrackState(),
                            index: dragState.step
                        )
                    )
                    .frame(width: 45, height: 45)
                    .opacity(0.7)

                    if dragState.mode == .copy {
                        Text("COPY")
                    } else {
                        Text("LOCK")
                    }
                }
                .position(dragState.position)
            }
        }
        .coordinateSpace(name: "SequenceViewZStack")
        .frame(height: 40 + PADDING * 2)
    }

    func startDrag(_ i: Int, _ drag: DragGesture.Value, _ mode: DragMode) {
        DispatchQueue.main.async {
            store.startDrag(
                source: .stepId(
                    StepId(
                        trackId: store.selectedTrack,
                        stepIndex: i
                    )
                ),
                dragMode: mode
            )
            dragState = DragState(step: i, position: drag.location, mode: mode)
        }
    }

    func endDrag() {
        store.endGlobalDrag()
        dragState = nil
    }
}

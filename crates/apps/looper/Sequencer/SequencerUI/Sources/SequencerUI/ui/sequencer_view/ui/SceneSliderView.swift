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
//  Created by Pedro Tacla Yamada on 9/3/2022.
//

import SwiftUI

let scenes = ["A", "B"]

struct SceneSliderView: View {
    @ObservedObject var sceneState: SceneState
    @EnvironmentObject var store: Store

    @State var isDragging: Int? = nil

    var body: some View {
        HStack {
            ContinuousButton(
                    action: {},
                    label: "A",
                    isSelected: false
            )
            .highPriorityGesture(makeDragGesture(sceneId: 0))
            .overlay(makeOverlay(sceneId: 0))
            .bindToParameterId(store: store, parameterId: .sceneButton(sceneId: 0), showSelectionOverlay: false)

            KnobSliderView(value: $sceneState.sceneSlider.value)
                .bindToParameter(store: store, parameter: sceneState.sceneSlider, showSelectionOverlay: false)

            ContinuousButton(
                    action: {},
                    label: "B",
                    isSelected: false
            )
            .highPriorityGesture(makeDragGesture(sceneId: 1))
            .overlay(makeOverlay(sceneId: 1))
            .bindToParameterId(store: store, parameterId: .sceneButton(sceneId: 1), showSelectionOverlay: false)
        }
    }

    func makeDragGesture(sceneId: Int) -> some Gesture {
        return DragGesture(coordinateSpace: .global)
            .onChanged { drag in
                self.store.focusState.sceneDragState = SceneDragState(
                    scene: sceneId,
                    position: drag.location,
                    startPosition: drag.startLocation
                )
                self.store.startDrag(source: .sceneId(SceneId(index: sceneId)), dragMode: .lock)
                store.objectWillChange.send()
                withAnimation {
                    isDragging = sceneId
                }
            }
            .onEnded { _ in
                self.store.endGlobalDrag()
                self.store.focusState.sceneDragState = nil
                isDragging = nil
            }
    }

    func makeOverlay(sceneId: Int) -> some View {
        ZStack(alignment: .center) {
            Rectangle()
                .fill(SequencerColors.blue.opacity(0.4))
                .border(SequencerColors.white, width: 1)
                .padding(1)
                .cornerRadius(BORDER_RADIUS)

            Text("Map scene")
                .bold()
                .frame(maxWidth: .infinity, alignment: .center)
        }
        .opacity(isDragging == sceneId ? 1.0 : 0.0)
        .frame(maxWidth: .infinity, maxHeight: .infinity)
        .padding(2)
        .allowsHitTesting(false)
    }
}

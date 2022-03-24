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

    var body: some View {
        HStack {
            TrackButton(
                action: {},
                label: "A",
                isSelected: false
            )
            .highPriorityGesture(makeDragGesture(sceneId: 0))
            .bindToParameterId(store: store, parameterId: .sceneButton(sceneId: 0), showSelectionOverlay: false)

            KnobSliderView(value: $sceneState.sceneSlider.value)
                .bindToParameter(store: store, parameter: sceneState.sceneSlider, showSelectionOverlay: false)

            TrackButton(
                action: {},
                label: "B",
                isSelected: false
            )
            .highPriorityGesture(makeDragGesture(sceneId: 1))
            .bindToParameterId(store: store, parameterId: .sceneButton(sceneId: 1), showSelectionOverlay: false)
        }
    }

    func makeDragGesture(sceneId: Int) -> some Gesture {
        return DragGesture(coordinateSpace: .global)
            .onChanged { drag in
                self.store.focusState.sceneDragState = SceneDragState(scene: sceneId, position: drag.location)
                self.store.startSceneDrag(sceneId)
                store.objectWillChange.send()
            }
            .onEnded { _ in
                self.store.endGlobalDrag()
                self.store.focusState.sceneDragState = nil
            }
    }
}

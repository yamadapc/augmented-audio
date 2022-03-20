//
//  SwiftUIView.swift
//
//
//  Created by Pedro Tacla Yamada on 9/3/2022.
//

import SwiftUI

struct SceneSliderView: View {
  @ObservedObject var sceneState: SceneState
  @EnvironmentObject var store: Store

    var body: some View {
        HStack {
            TrackButton(action: {}, label: "A", isSelected: false)
            KnobSliderView(value: $sceneState.sceneSlider.value)
              .bindToParameter(store: store, parameter: sceneState.sceneSlider)
            TrackButton(action: {}, label: "B", isSelected: false)
        }
    }
}

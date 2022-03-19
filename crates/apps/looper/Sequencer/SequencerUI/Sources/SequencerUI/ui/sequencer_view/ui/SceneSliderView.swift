//
//  SwiftUIView.swift
//
//
//  Created by Pedro Tacla Yamada on 9/3/2022.
//

import SwiftUI

struct SceneSliderView: View {
    @State var value: Double = 0

    var body: some View {
        HStack {
            TrackButton(action: {}, label: "A", isSelected: false)
            KnobSliderView(value: $value)
            TrackButton(action: {}, label: "B", isSelected: false)
        }
    }
}

struct SceneSliderView_Previews: PreviewProvider {
    static var previews: some View {
        SceneSliderView()
    }
}

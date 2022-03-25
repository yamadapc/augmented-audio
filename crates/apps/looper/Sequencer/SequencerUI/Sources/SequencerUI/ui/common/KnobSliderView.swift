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

enum KnobSliderStyle {
    case horizontal, vertical
}

struct KnobSliderView: View {
    @Binding var value: Float
    var defaultValue: Float = -1.0

    var style: KnobSliderStyle = .horizontal
    var tickColor = SequencerColors.black.opacity(0.6)
    var railColor = SequencerColors.black.opacity(0.8)
    var handleColor = SequencerColors.black
    var handleAccentColor = SequencerColors.blue

    var body: some View {
        let handleWidth = 30.0
        GeometryReader { geometry in
            ZStack {
                // Rail
                Rectangle()
                    .fill(railColor)
                    .frame(maxHeight: style == .horizontal ? 5 : .infinity)
                    .frame(maxWidth: style == .horizontal ? .infinity : 5)
                    .position(x: geometry.size.width / 2.0, y: geometry.size.height / 2.0)

                // Ticks
                ForEach(0 ..< 11) { i in
                    let isLarge = i % 2 == 0

                    Rectangle()
                        .fill(tickColor)
                        .frame(
                            width: style == .horizontal
                                ? 2
                                : isLarge ? geometry.size.width * 0.7 : geometry.size.width * 0.5,
                            height: style == .horizontal
                                ? isLarge ? geometry.size.height * 0.7 : geometry.size.height * 0.5
                                : 2
                        )
                        .position(
                            x: style == .horizontal
                                ? (CGFloat(i) / 10.0) * geometry.size.width
                                : geometry.size.width / 2.0,
                            y: style == .horizontal
                                ? geometry.size.height / 2.0
                                : CGFloat(i) / 10.0 * geometry.size.height
                        )
                }.frame(maxWidth: .infinity)

                // Handle
                ZStack {
                    Rectangle()
                        .fill(handleColor)
                        .frame(maxWidth: style == .horizontal ? handleWidth : .infinity)
                        .frame(maxHeight: style == .horizontal ? .infinity : handleWidth)

                    Rectangle()
                        .fill(handleAccentColor)
                        .frame(
                            width: style == .horizontal ? 2 : 0.7 * geometry.size.width,
                            height: style == .horizontal ? 0.7 * geometry.size.height : 2
                        )
                }
                .position(x: geometry.size.width / 2.0, y: geometry.size.height / 2.0)
                .transformEffect(
                    style == .horizontal
                        ? .init(translationX: Double(value) * geometry.size.width / 2.0, y: 0)
                        : .init(translationX: 0, y: Double(0.5 - value) * geometry.size.height)
                )
                .gesture(
                    TapGesture(count: 2).onEnded {
                        self.value = defaultValue
                    }
                )
                .gesture(
                    DragGesture(minimumDistance: 0)
                        .onChanged { onDrag($0, geometry) }
                )
            }
            .contentShape(Rectangle())
            .frame(maxWidth: .infinity)
            .frame(maxHeight: .infinity)
            .gesture(DragGesture(minimumDistance: 0).onChanged { onDrag($0, geometry) })
        }
        .padding(
            style == .horizontal
                ? EdgeInsets(top: PADDING, leading: handleWidth / 2.0, bottom: PADDING, trailing: handleWidth / 2.0)
                : EdgeInsets(top: PADDING, leading: 0, bottom: 0, trailing: 0)
        )
        .frame(maxHeight: style == .horizontal ? 80 : .infinity)
    }

    func onDrag(_ drag: DragGesture.Value, _ geometry: GeometryProxy) {
        if style == .horizontal {
            let value = (drag.location.x / geometry.size.width) * 2.0 + -1.0
            self.value = Float(min(max(value, -1.0), 1.0))
        } else {
            let value = (drag.location.y / geometry.size.height)
            self.value = (1.0 - Float(min(max(value, 0.0), 1.0)))
        }
    }
}

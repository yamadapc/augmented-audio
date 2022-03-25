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

struct Triangle: Shape {
    func path(in rect: CGRect) -> Path {
        var path = Path()

        path.move(to: CGPoint(x: rect.minX, y: rect.maxY))
        path.addLine(to: CGPoint(x: rect.midX, y: rect.minY))
        path.addLine(to: CGPoint(x: rect.maxX, y: rect.maxY))
        path.addLine(to: CGPoint(x: rect.minX, y: rect.maxY))

        return path
    }
}

struct SliceHandleView: View {
    var inverted: Bool = false

    var body: some View {
        ZStack {
            GeometryReader { geometry in
                Triangle()
                    .rotationEffect(Angle(degrees: inverted ? -90.0 : 90.0))
                    .frame(width: 10, height: 10)
                    .position(x: inverted ? -5 : 5, y: inverted ? geometry.size.height : 0)

                Rectangle()
                    .frame(width: 1.0)
                    .frame(maxHeight: .infinity)
                    .position(x: 0, y: geometry.size.height / 2.0)
            }
        }
    }
}

struct SourceParametersOverlayView: View {
    @ObservedObject var sourceParameters: SourceParametersState
    @ObservedObject var start: SourceParameter
    @ObservedObject var end: SourceParameter

    init(sourceParameters: SourceParametersState) {
        self.sourceParameters = sourceParameters
        start = sourceParameters.start
        end = sourceParameters.end
    }

    var body: some View {
        GeometryReader { geometry in
            ZStack {
                SliceHandleView()
                    .transformEffect(.init(
                        translationX: CGFloat(sourceParameters.start.value) * geometry.size.width,
                        y: 0
                    ))
                    .gesture(
                        buildGesture(
                            parameter: sourceParameters.start,
                            geometry: geometry,
                            valueMin: 0,
                            valueMax: sourceParameters.end.value
                        )
                    )

                SliceHandleView(inverted: true)
                    .transformEffect(.init(
                        translationX: CGFloat(sourceParameters.end.value) * geometry.size.width,
                        y: 0
                    ))
                    .gesture(
                        buildGesture(
                            parameter: sourceParameters.end,
                            geometry: geometry,
                            valueMin: sourceParameters.start.value,
                            valueMax: .infinity
                        )
                    )
            }
        }
    }

    func buildGesture(parameter: SourceParameter, geometry: GeometryProxy, valueMin: Float, valueMax: Float) -> some Gesture {
        return DragGesture(minimumDistance: 0)
            .onChanged { dragValue in
                var percX = dragValue.location.x / geometry.size.width
                percX = max(min(percX, 1.0), 0.0)
                percX = max(min(percX, CGFloat(valueMax)), CGFloat(valueMin))
                parameter.value = Float(percX)
            }
    }
}

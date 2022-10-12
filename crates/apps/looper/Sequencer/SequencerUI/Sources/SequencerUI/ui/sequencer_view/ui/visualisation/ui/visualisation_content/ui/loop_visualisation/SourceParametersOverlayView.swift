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
                let size = 30.0
                Triangle()
                    .rotationEffect(Angle(degrees: inverted ? -90.0 : 90.0))
                    .frame(width: 30, height: 30)
                    .position(x: inverted ? -size / 2.0 : size / 2.0, y: inverted ? geometry.size.height - size / 2.0 : size / 2.0)

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
            ZStack(alignment: .topLeading) {
                // Touch drag layer
                Rectangle()
                    .foregroundColor(SequencerColors.black.opacity(0))
                    .frame(maxWidth: .infinity, maxHeight: .infinity)
                    .gesture(
                        DragGesture()
                            .onChanged { dragValue in
                                print(dragValue)
                            }
                    )

                SliceHandleView()
                    .position(
                        x: CGFloat(sourceParameters.start.value) * geometry.size.width + geometry.size.width / 2.0,
                        y: geometry.size.height / 2.0
                    )
                    .gesture(
                        buildGesture(
                            parameter: sourceParameters.start,
                            geometry: geometry,
                            valueMin: 0,
                            valueMax: sourceParameters.end.value
                        )
                    )

                SliceHandleView(inverted: true)
                    .position(
                        x: CGFloat(sourceParameters.end.value) * geometry.size.width + geometry.size.width / 2.0,
                        y: geometry.size.height / 2.0
                    )
                    .gesture(
                        buildGesture(
                            parameter: sourceParameters.end,
                            geometry: geometry,
                            valueMin: sourceParameters.start.value,
                            valueMax: .infinity
                        )
                    )
            }
            .frame(width: geometry.size.width, height: geometry.size.height)
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
    }

    func buildGesture(parameter: SourceParameter, geometry: GeometryProxy, valueMin: Float, valueMax: Float) -> some Gesture {
        return DragGesture()
            .onChanged { dragValue in
                var percX = dragValue.location.x / geometry.size.width
                percX = max(min(percX, 1.0), 0.0)
                percX = max(min(percX, CGFloat(valueMax)), CGFloat(valueMin))
                parameter.value = Float(percX)
            }
    }
}

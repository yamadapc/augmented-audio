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

// TODO: - This is too hard to optimise. I will try to rewrite it using AppKit.
// If it is not faster then I will have to rewrite this app using yet another GUI solution. :(

import AVFAudio
import SwiftUI

public enum KnobStyle {
    case normal, center
}

struct TrackBackgroundView {
    var trackColor: Color
    var strokeWidth: Double
    var radius: Double

    var body: some View {
        return Circle()
            .trim(from: 0.0, to: 0.75)
            .rotation(Angle(radians: (1.5 * 0.25) * .pi * 2.0))
            .stroke(trackColor, lineWidth: strokeWidth)
            .frame(width: radius * 2, height: radius * 2)
    }
}

struct TrackSliderView {
    var style: KnobStyle
    var value: Double
    var strokeWidth: Double
    var radius: Double
    var color: Color

    var body: some View {
        switch self.style {
        case .normal:
            return Circle()
                .trim(from: 0.0, to: 0.75 * value)
                .rotation(Angle(radians: (1.5 * 0.25) * .pi * 2.0))
                .stroke(color, lineWidth: strokeWidth)
                .frame(width: radius * 2, height: radius * 2)
        case .center:
            let start = 0.0
            let end = 0.375 * fabs(value)
            let realSweepAngle = 0.75 * .pi
            let rotation = value >= 0
                ? 0.75 * .pi * 2.0 // if the value is positive, we're rotated up to 12 o'clock
                : 0.75 * .pi * 2.0 + realSweepAngle * value // if the value is negative we're rotated backwards
            return Circle()
                .trim(from: start, to: end)
                .rotation(Angle(radians: rotation))
                .stroke(color, lineWidth: strokeWidth)
                .frame(width: radius * 2, height: radius * 2)
        }
    }
}

struct KnobRenderOptions {
    let renderLine: Bool
    let renderThumb: Bool
}

struct KnobView: View {
    var radius: Double = 30
    var label: String = " "
    var strokeWidth: Double = 5
    var onChanged: ((Double) -> Void)? = nil
    var onEnded: (() -> Void)? = nil
    var style: KnobStyle = .normal
    var renderOptions: KnobRenderOptions = .init(
        renderLine: true,
        renderThumb: false
    )
    var formatValue: ((Double) -> String)? = nil
    var value: Double = 0.0

    func style(_ style: KnobStyle) -> KnobView {
        KnobView(
            radius: radius,
            label: label,
            strokeWidth: strokeWidth,
            onChanged: onChanged,
            onEnded: onEnded,
            style: style,
            renderOptions: renderOptions,
            formatValue: formatValue,
            value: value
        )
    }

    var body: some View {
        let color = SequencerColors.blue
        let trackColor = SequencerColors.black1

        if #available(macOS 11, *) {
            MacKnobView(
                value: Float(self.value),
                label: label,
                formattedValue: getFormattedValue(),
                style: style
            )
            .frame(width: radius * 2 + 10, height: radius * 2 + 40)
            .contentShape(Rectangle())
            .gesture(
                DragGesture(minimumDistance: 3.0)
                    .onChanged { value in self.onGestureChanged(value) }
                    .onEnded { _ in self.onEnded?() },
                including: .all
            )
        } else {
            VStack {
                Text(self.getFormattedValue())

                ZStack {
                    Rectangle()
                        .fill(Color.red.opacity(0))
                        .position(x: radius + 5, y: radius - 5)
                        .frame(width: radius * 2 + 10, height: radius * 2 + 10)
                    ZStack {
                        TrackBackgroundView(
                            trackColor: trackColor,
                            strokeWidth: strokeWidth,
                            radius: radius
                        ).body
                        TrackSliderView(
                            style: style,
                            value: value,
                            strokeWidth: strokeWidth,
                            radius: radius,
                            color: color
                        ).body

                        renderThumbAndPointer()
                    }
                }
                .frame(width: radius * 2, height: radius * 2)
                .contentShape(Rectangle())
                .gesture(
                    DragGesture(minimumDistance: 3.0)
                        .onChanged { value in self.onGestureChanged(value) }
                        .onEnded { _ in self.onEnded?() },
                    including: .all
                )
                .fixedSize()

                Text(label)
                    .fixedSize()
                    .allowsTightening(true)
                    .lineLimit(1)
            }
        }
    }

    fileprivate func getFormattedValue() -> String {
        if let formatValue = formatValue {
            return formatValue(value)
        }
        return String(format: "%.2f", value)
    }

    fileprivate func renderThumbAndPointer() -> some View {
        let startAngle: Double = style == .normal ? 0.75 * .pi : 0.75 * .pi * 2
        let thumbAngle: Double = startAngle + value * realSweepAngle()
        let centerCoordinate: Double = radius + strokeWidth
        let knobPosition = CGPoint(
            x: centerCoordinate + radius * cos(thumbAngle),
            y: centerCoordinate + radius * sin(thumbAngle)
        )

        return ZStack {
            if renderOptions.renderLine {
                Path { builder in
                    let pathPosition = CGPoint(
                        x: centerCoordinate + (radius - strokeWidth * 1.5) * cos(thumbAngle),
                        y: centerCoordinate + (radius - strokeWidth * 1.5) * sin(thumbAngle)
                    )

                    builder.move(to: CGPoint(x: centerCoordinate, y: centerCoordinate))
                    builder.addLine(to: pathPosition)
                }.stroke(SequencerColors.white.opacity(0.7), lineWidth: strokeWidth / 2)
            }

            if renderOptions.renderThumb {
                Circle()
                    .fill(SequencerColors.white)
                    .frame(width: strokeWidth * 2, height: strokeWidth * 2)
                    .shadow(radius: SHADOW_RADIUS)
                    .position(
                        x: knobPosition.x,
                        y: knobPosition.y
                    )
            }
        }
    }

    /// Actual rotation in radians between 0 and 1 for this style
    func realSweepAngle() -> Double {
        return style == .normal
            ? 1.5 * .pi
            : 0.75 * .pi
    }

    /// The value is calculated as if this was .normal style and then converted
    func onGestureChanged(_ value: DragGesture.Value) {
        let location = value.location

        // If we are using the macOS native view, then
        // the Y coordinate is offset by 20px.
        let centerX = radius
        var centerY = radius
        if #available(macOS 11, *) {
            centerY += 20
        }

        let startAngle = .pi * 0.75
        let sweepAngle = 0.75 * .pi * 2.0

        var angle = atan2(
            location.y - centerY,
            location.x - centerX
        ) - startAngle // this offsets the angle so 0 is at startAngle

        // This fixes the angle so it goes from 0 to 2.pi radius
        if angle < 0 {
            angle = 2 * .pi + angle
        }

        // This truncates values under the knob so they either snap to start or end
        // In reality we should cancel the gesture so that there're no jumps
        if angle > sweepAngle, angle < sweepAngle + 0.25 * .pi {
            angle = sweepAngle
        } else if angle > sweepAngle {
            angle = 0
        }

        var newValue = angle / sweepAngle
        newValue = max(min(newValue, 1), 0)

        let styledValue = style == .normal
            ? newValue
            : newValue * 2 + -1

        onChanged?(styledValue)
    }
}

struct KnobView_Previews: PreviewProvider {
    static var previews: some View {
        Group {
            KnobView(radius: 20, label: "Normal", strokeWidth: 5)
                .padding(30)
            KnobView(radius: 30, label: "Center", strokeWidth: 5)
                .style(.center)
        }
    }
}

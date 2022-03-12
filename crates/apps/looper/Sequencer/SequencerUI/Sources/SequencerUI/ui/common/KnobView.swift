//
//  KnobView.swift
//  Sequencer
//
//  Created by Pedro Tacla Yamada on 28/2/2022.
//

import AVFAudio
import SwiftUI

enum KnobStyle {
    case normal, center
}

struct TrackBackgroundView: View {
    var trackColor: Color
    var strokeWidth: Double
    var radius: Double

    var body: some View {
        Circle()
            .trim(from: 0.0, to: 0.75)
            .rotation(Angle(radians: (1.5 * 0.25) * .pi * 2.0))
            .stroke(trackColor, lineWidth: strokeWidth)
            .frame(width: radius * 2, height: radius * 2)
    }
}

struct TrackSliderView: View {
    var style: KnobStyle
    var value: Double
    var strokeWidth: Double
    var radius: Double
    var color: Color

    var body: some View {
        switch self.style {
        case .normal:
            Circle()
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
            Circle()
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
    var style: KnobStyle = .normal
    var renderOptions: KnobRenderOptions = .init(
        renderLine: true,
        renderThumb: false
    )

    @State var value: Double = 1.0

    func style(_ style: KnobStyle) -> KnobView {
        KnobView(
            radius: radius,
            label: label,
            strokeWidth: strokeWidth,
            onChanged: onChanged,
            style: style,
            value: value
        )
    }

    var body: some View {
        let color = SequencerColors.blue
        let trackColor = SequencerColors.black1

        VStack {
            Text(String(format: "%.2f", value))

            ZStack {
                Rectangle()
                    .fill(Color.red.opacity(0))
                    .position(x: radius + 5, y: radius - 5)
                    .frame(width: radius * 2 + 10, height: radius * 2 + 10)

                ZStack {
                    TrackBackgroundView(trackColor: trackColor, strokeWidth: strokeWidth, radius: radius)
                    TrackSliderView(
                        style: style,
                        value: value,
                        strokeWidth: strokeWidth,
                        radius: radius,
                        color: color
                    )

                    renderThumbAndPointer()
                }
            }
            .frame(width: radius * 2, height: radius * 2)
            .contentShape(Rectangle())
            .gesture(
                DragGesture(minimumDistance: 3.0)
                    .onChanged { value in self.onGestureChanged(value) },
                including: .all
            )

            Text(label)
        }
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
                    .shadow(radius: 3)
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
        let centerX = radius
        let centerY = radius

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

        self.value = styledValue
        onChanged?(newValue)
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

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

import AppKit
import SwiftUI

/**
 * This struct represents an Arc that will be drawn for either the background or the filled track of a knob
 */
struct MacKnobPathArc {
    let center: CGPoint
    let startAngle: Double
    let endAngle: Double
    let radius: Double
}

/**
 * Calculate the arc for a given value between 0-1.
 *
 * This arc will leave a `0.25 * .pi` radians (45º) gap at the bottom of *each side* the knob and
 * be filled according to the `value` input.
 *
 * For example, if the value is 0.5, the knob should be filled between 135º and 270º.
 *
 * Drawing is inverted so angles are negative.
 */
func buildValueTrack(
    radius: Double,
    strokeWidth: Double,
    value: Double
) -> MacKnobPathArc {
    let center = CGPoint(
        x: radius + strokeWidth,
        y: radius + strokeWidth
    )

    let start = 0.0 - 0.75 * .pi
    let end = start - (0.75 * .pi * 2.0) * value

    return MacKnobPathArc(
        center: center,
        startAngle: start,
        endAngle: end,
        radius: radius
    )
}

/**
 * Calculate the arc for a given value between -1 and +1 for a center knob.
 *
 * This arc will leave a 90º gap between start/end. The value `0` will be positioned mid-way between
 * 135º and 405º, at 270º past the right x-axis start.
 *
 * The path is drawn by calculating what the `value * sweepAngle` total angle is between start/end
 * then starting at the `0` point at 270º.
 *
 * If the value is negative, we rotate the *starting angle* counterclowise by this amount.
 */
func buildCenterValueTrack(
    radius: Double,
    strokeWidth: Double,
    value: Double
) -> MacKnobPathArc {
    let sweepAngle = getSweepAngle(style: .center)

    let center = CGPoint(
        x: radius + strokeWidth,
        y: radius + strokeWidth
    )

    // This knob is a centered knob, so 0 is at the middle and -1/+1 at each
    // side. We will rotate the start angle either to the middle or back
    var start = 0.0 // - 0.75 * .pi
    if value >= 0 {
        start -= 0.75 * .pi * 2.0
    } else {
        start -= 0.75 * .pi * 2.0 + sweepAngle * value
    }
    let end = start - sweepAngle * fabs(value)

    return MacKnobPathArc(
        center: center,
        startAngle: start,
        endAngle: end,
        radius: radius
    )
}

/**
 * This returns the angle in radians that the value `1.0` represents.
 *
 * For a normal knob, the track will start at 135º and end at 405º, counting from 0º at the right side x-axis.
 * The sweep-angle should be 270º degrees. This is the angle between start and end of the track.
 *
 * For a centric knob that goes from -1 to +1 this is halfed.
 */
func getSweepAngle(style: KnobStyle) -> Double {
    return style == .normal
        ? 1.5 * .pi
        : 0.75 * .pi
}

/**
 * This represents the two points a knob thumb go. This is the line from the center of the knob to
 * the current value.
 */
struct MacKnobPointerPath {
    let start: CGPoint
    let end: CGPoint
}

/**
 * Calculate the center & current knob positions for the knob.
 *
 * This calculation works in the following way.
 * First we determine the start angle of the track, depending on the knob style. This is either 135º or 270º for
 * normal and center knobs respectively.
 *
 * Then, we calculate the angle at which the thumb will be from the 0º by subtracting the start angle by the
 * `value` times the total `sweepAngle` rotation for this style.
 *
 * Then, we calculate the positions of both the pointer along the track and the center of the path considering
 * a `0.5 * strokeWidth` margin between the line and the circle edge and offsetting positions to account
 * for the circle line width.
 */
func buildPointerPath(
    radius: Double,
    style: KnobStyle,
    strokeWidth: Double,
    value: Double
) -> MacKnobPointerPath {
    let startAngle: Double = style == .normal ? -0.75 * .pi : -0.75 * .pi * 2.0
    let thumbAngle: Double = startAngle - value * getSweepAngle(style: style)
    let centerCoordinate: Double = radius + strokeWidth

    let pathPosition = CGPoint(
        x: centerCoordinate + (radius - strokeWidth * 1.5) * cos(thumbAngle),
        y: centerCoordinate + (radius - strokeWidth * 1.5) * sin(thumbAngle)
    )

    return MacKnobPointerPath(
        start: CGPoint(x: centerCoordinate, y: centerCoordinate),
        end: pathPosition
    )
}

/**
 * This is required because SwiftUI has poor performance for rendering the knob, found in `KnobView`.
 *
 * This implementation has significantly better performance and is much more responsive, because updating
 * the SwiftUI implementation on drag causes a lot of layout operations around its ZStack and VStacks.
 *
 * It might be possible to optimise the SwiftUI implementation as well.
 */
@available(macOS 11, *)
class MacKnobNSView: NSView {
    var value: Float = 0.0 {
        didSet {
            needsDisplay = true
        }
    }

    var style: KnobStyle = .normal
    var radius = 30.0
    var strokeWidth = 5.0

    override func draw(_: NSRect) {
        let context = NSGraphicsContext.current?.cgContext

        context?.translateBy(x: 0, y: -10)

        context?.saveGState()
        let bgPath = buildPath(1.0)
        context?.addPath(bgPath)
        context?.setLineWidth(strokeWidth)
        context?.setStrokeColor(SequencerColors.black1.cgColor!)
        context?.strokePath()
        context?.restoreGState()

        context?.saveGState()
        let trackSliderPath = buildFilledValuePath()
        context?.addPath(trackSliderPath)
        context?.setLineWidth(strokeWidth)
        context?.setStrokeColor(SequencerColors.blue.cgColor!)
        context?.strokePath()
        context?.restoreGState()

        context?.saveGState()
        context?.addPath(buildPointer())
        context?.setLineWidth(strokeWidth / 2.0)
        context?.setStrokeColor(SequencerColors.white.cgColor!)
        context?.strokePath()
        context?.restoreGState()
    }

    func buildPointer() -> CGPath {
        let pointerPath = buildPointerPath(
            radius: radius,
            style: style,
            strokeWidth: strokeWidth,
            value: Double(value)
        )
        let path = CGMutablePath()
        path.move(to: pointerPath.start)
        path.addLine(to: pointerPath.end)
        return path
    }

    func buildFilledValuePath() -> CGPath {
        switch style {
        case .normal:
            return buildPath(Double(value))
        case .center:
            return buildCenterPath()
        }
    }

    func buildCenterPath() -> CGPath {
        let arc = buildCenterValueTrack(
            radius: radius,
            strokeWidth: strokeWidth,
            value: Double(value)
        )
        let path = CGMutablePath()
        path.addArc(
            center: arc.center,
            radius: arc.radius,
            startAngle: arc.startAngle,
            endAngle: arc.endAngle,
            clockwise: true
        )

        return path
    }

    func buildPath(_ value: Double) -> CGPath {
        let arc = buildValueTrack(
            radius: radius,
            strokeWidth: strokeWidth,
            value: value
        )

        let path = CGMutablePath()
        path.addArc(
            center: arc.center,
            radius: arc.radius,
            startAngle: arc.startAngle,
            endAngle: arc.endAngle,
            clockwise: true
        )

        return path
    }
}

@available(macOS 11, *)
class MacKnobViewWithText: NSView {
    var valueTextView: NSTextView
    var labelView: NSTextView
    var knobView: MacKnobNSView
    var style: KnobStyle = .normal {
        didSet {
            knobView.style = style
        }
    }

    var value: Float {
        get { knobView.value }
        set {
            knobView.value = newValue
        }
    }

    var label: String = "" {
        didSet {
            labelView.string = label
        }
    }

    var formattedValue: String = "" {
        didSet {
            valueTextView.string = formattedValue
        }
    }

    init() {
        knobView = MacKnobNSView()
        valueTextView = NSTextView()
        labelView = NSTextView()
        super.init(frame: NSRect.zero)
        setupSubViews()
    }

    required init?(coder: NSCoder) {
        valueTextView = NSTextView()
        labelView = NSTextView()
        knobView = MacKnobNSView()
        super.init(coder: coder)
        setupSubViews()
    }

    func setupSubViews() {
        valueTextView.isEditable = false
        labelView.isEditable = false
        valueTextView.alignment = .center
        labelView.alignment = .center
        valueTextView.drawsBackground = false
        labelView.drawsBackground = false

        knobView.wantsLayer = true

        addSubview(valueTextView)
        addSubview(labelView)
        addSubview(knobView)

        layout()
    }

    override func layout() {
        let (valueFrame, remaining1) = frame.divided(
            atDistance: 20, from: .maxYEdge
        )
        valueTextView.frame = valueFrame
        let (labelFrame, remaining2) = remaining1.divided(
            atDistance: 20, from: .minYEdge
        )
        labelView.frame = labelFrame

        knobView.frame = remaining2
    }
}

@available(macOS 11, *)
struct MacKnobView: NSViewRepresentable {
    var value: Float
    var label: String
    var formattedValue: String
    var style: KnobStyle

    typealias NSViewType = MacKnobViewWithText

    func makeNSView(context _: Context) -> MacKnobViewWithText {
        let view = MacKnobViewWithText()
        view.value = value
        view.label = label
        view.formattedValue = formattedValue
        view.style = style
        return view
    }

    func updateNSView(_ nsView: MacKnobViewWithText, context _: Context) {
        nsView.value = value
        nsView.label = label
        nsView.formattedValue = formattedValue
        nsView.style = style
        nsView.needsDisplay = true
    }
}

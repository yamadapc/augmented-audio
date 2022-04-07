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

func buildBackgroundLayer() {}

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
        let startAngle: Double = style == .normal ? -0.75 * .pi : -0.75 * .pi * 2.0
        let thumbAngle: Double = startAngle - Double(value) * realSweepAngle()
        let centerCoordinate: Double = radius + strokeWidth

        let path = CGMutablePath()

        let pathPosition = CGPoint(
            x: centerCoordinate + (radius - strokeWidth * 1.5) * cos(thumbAngle),
            y: centerCoordinate + (radius - strokeWidth * 1.5) * sin(thumbAngle)
        )
        path.move(to: CGPoint(x: centerCoordinate, y: centerCoordinate))
        path.addLine(to: pathPosition)

        return path
    }

    func buildFilledValuePath() -> CGPath {
        switch style {
        case .normal:
            return buildPath(Double(getNormal()))
        case .center:
            return buildCenterPath()
        }
    }

    func buildCenterPath() -> CGPath {
        let value = Double(getNormal())
        let path = CGMutablePath()

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
            start -= 0.75 * .pi * 2.0 + realSweepAngle() * value
        }
        let end = start - realSweepAngle() * fabs(value)

        path.addArc(
            center: center,
            radius: radius,
            startAngle: start,
            endAngle: end,
            clockwise: true
        )

        return path
    }

    func buildPath(_ value: Double) -> CGPath {
        let path = CGMutablePath()

        let center = CGPoint(
            x: radius + strokeWidth,
            y: radius + strokeWidth
        )

        let start = 0.0 - 0.75 * .pi
        let end = start - (0.75 * .pi * 2.0) * value

        path.addArc(
            center: center,
            radius: radius,
            startAngle: start,
            endAngle: end,
            clockwise: true
        )

        return path
    }

    /// Actual rotation in radians between 0 and 1 for this style
    func realSweepAngle() -> Double {
        return style == .normal
            ? 1.5 * .pi
            : 0.75 * .pi
    }

    func getNormal() -> Float {
        return value
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

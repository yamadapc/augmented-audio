//
//  KnobView.swift
//  Sequencer
//
//  Created by Pedro Tacla Yamada on 28/2/2022.
//

import SwiftUI

struct KnobView: View {
  var radius: Double = 50
  var strokeWidth: Double = 10
  var onChanged: ((Double) -> Void)? = nil

  @State var value: Double = 1.0

  var body: some View {
    let color = SequencerColors.blue
    let trackColor = SequencerColors.black1

    VStack {
      Text(String(format: "%.2f", value))

      ZStack {
        Circle()
          .fill(color)
          .frame(width: 4, height: 4)
          .position(x: radius, y: radius)
        Circle()
          .trim(from: 0.0, to: 0.75)
          .rotation(Angle(radians: (1.5 * 0.25) * .pi * 2.0))
          .stroke(trackColor, lineWidth: strokeWidth)
          .frame(width: radius * 2, height: radius * 2)

        Circle()
          .trim(from: 0.0, to: 0.75 * value)
          .rotation(Angle(radians: (1.5 * 0.25) * .pi * 2.0))
          .stroke(color, lineWidth: strokeWidth)
          .frame(width: radius * 2, height: radius * 2)
      }
      .frame(width: radius * 2, height: radius * 2)
      .gesture(
        DragGesture(minimumDistance: 0.0)
          .onChanged({ value in
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
            if angle > sweepAngle && angle < sweepAngle + 0.25 * .pi {
              angle = sweepAngle
            } else if angle > sweepAngle {
              angle = 0
            }

            var newValue = angle / sweepAngle
            newValue = max(min(newValue, 1), 0)

            self.value = newValue
            onChanged?(newValue)
          })
        )
    }
  }
}

struct KnobView_Previews: PreviewProvider {
  static var previews: some View {
    KnobView(radius: 20, strokeWidth: 5)
      .padding(30)
  }
}

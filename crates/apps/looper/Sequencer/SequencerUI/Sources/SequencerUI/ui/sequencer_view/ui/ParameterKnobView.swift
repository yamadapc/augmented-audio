//
//  File.swift
//
//
//  Created by Pedro Tacla Yamada on 17/3/2022.
//

import SwiftUI

struct ParameterKnobView: View {
    @ObservedObject var parameter: FloatParameter

    var body: some View {
        KnobView(
            label: parameter.label,
            onChanged: { newValue in
                    let mappedValue = parameterFromKnobValue(knobValue: newValue)
                    parameter.value = mappedValue
            },
            formatValue: { _knobValue in
              String(format: "%.2f", parameter.value)
            },
            value: Double(
              parameterToKnobValue()
            )
        )
        .style(parameter.style)
        .gesture(
            TapGesture(count: 2).onEnded {
                parameter.value = parameter.defaultValue
            }
        )
    }

  func parameterFromKnobValue(knobValue: Double) -> Float {
    let result = Float(
      parameter.style == .center
        ? (knobValue + 1.0) / 2.0
        : knobValue
    ) * (parameter.range.1 - parameter.range.0) + parameter.range.0

    print("knobValue=\(knobValue) result=\(result)")
    return result  }

  func parameterToKnobValue() -> Float {
    // number between 0-1
    let value = (parameter.value - parameter.range.0) / (parameter.range.1 - parameter.range.0)
    if parameter.style == .center {
        return value * 2.0 + -1.0
    }

    return value
  }
}

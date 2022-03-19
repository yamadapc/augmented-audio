import SwiftUI

struct ParameterKnobView<ParameterId>: View {
    @ObservedObject var parameter: FloatParameter<ParameterId>
    @EnvironmentObject var store: Store

    var body: some View {
        KnobView(
            label: parameter.label,
            onChanged: { newValue in
                let mappedValue = parameterFromKnobValue(knobValue: newValue)
                parameter.setValue(mappedValue)
            },
            onEnded: {
                store.endParameterLock(parameter)
            },
            formatValue: { _ in
                String(format: "%.2f", parameter.parameterLockProgress?.newValue ?? parameter.value)
            },
            value: Double(
                parameterToKnobValue()
            )
        )
        .style(parameter.style)
        .gesture(
            TapGesture(count: 2).onEnded {
                parameter.setValue(parameter.defaultValue)
            }
        )
        .bindToParameter(store: store, parameter: parameter)
        .overlay(
            ParameterLockAnimationView(
                focusState: store.focusState,
                parameterId: parameter.globalId
            ),
            alignment: .center
        )
    }

    func parameterFromKnobValue(knobValue: Double) -> Float {
        let (min, max) = parameter.range
        // number between 0-1
        let result = Float(
            parameter.style == .center
                ? (knobValue + 1.0) / 2.0
                : knobValue
        ) * (max - min) + min

        return result
    }

    func parameterToKnobValue() -> Float {
        let value = parameter.parameterLockProgress?.newValue ?? parameter.value
        let (min, max) = parameter.range
        // number between 0-1
        let knobValue = (value - min) / (max - min)
        if parameter.style == .center {
            return knobValue * 2.0 + -1.0
        }

        return knobValue
    }
}

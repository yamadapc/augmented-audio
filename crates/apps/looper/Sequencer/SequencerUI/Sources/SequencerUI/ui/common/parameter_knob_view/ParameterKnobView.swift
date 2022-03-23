import SwiftUI

protocol KnobParameterLike: ObservableObject {
    var label: String { get }
    var globalId: ObjectId { get }
    var style: KnobStyle { get }

    func formatValue() -> String
    func setKnobValue(_ value: Double)
    func toKnobValue() -> Float
    func resetDefault()
    func endParameterLock(_ store: Store)
}

extension IntParameter: KnobParameterLike {
    var globalId: ObjectId { id }
    var style: KnobStyle { .normal }

    func formatValue() -> String {
        "\(value)"
    }

    func setKnobValue(_ newValue: Double) {
        value = Int(Float(newValue) * Float(maximum))
    }

    func resetDefault() {
        value = 0
    }

    func toKnobValue() -> Float {
        let result = Float(value) / Float(max(maximum, 1))
        return result
    }

    func endParameterLock(_: Store) {}
}

extension FloatParameter: KnobParameterLike {
    func formatValue() -> String {
        return String(format: "%.2f", parameterLockProgress?.newValue ?? value)
    }

    func endParameterLock(_ store: Store) {
        store.endParameterLock(self)
    }

    func resetDefault() {
        setValue(defaultValue)
    }

    func toKnobValue() -> Float {
        let value = parameterLockProgress?.newValue ?? value
        let (min, max) = range
        // number between 0-1
        let knobValue = (value - min) / (max - min)
        if style == .center {
            return knobValue * 2.0 + -1.0
        }

        return knobValue
    }

    func setKnobValue(_ newValue: Double) {
        let mappedValue = fromKnobValue(knobValue: newValue)
        setValue(mappedValue)
    }

    func fromKnobValue(knobValue: Double) -> Float {
        let (min, max) = range
        // number between 0-1
        let result = Float(
            style == .center
                ? (knobValue + 1.0) / 2.0
                : knobValue
        ) * (max - min) + min

        return result
    }
}

struct ParameterKnobView<ParameterT: KnobParameterLike>: View {
    @ObservedObject var parameter: ParameterT
    @EnvironmentObject var store: Store

    var body: some View {
        KnobView(
            label: parameter.label,
            onChanged: { newValue in
                parameter.setKnobValue(newValue)
            },
            onEnded: {
                parameter.endParameterLock(store)
            },
            formatValue: { _ in
                parameter.formatValue()
            },
            value: Double(parameter.toKnobValue())
        )
        .style(parameter.style)
        .gesture(
            TapGesture(count: 2).onEnded {
                parameter.resetDefault()
            }
        )
        .bindToParameterId(store: store, parameterId: parameter.globalId)
        .overlay(
            ParameterLockAnimationView(
                focusState: store.focusState,
                parameterId: parameter.globalId
            ),
            alignment: .center
        )
    }
}

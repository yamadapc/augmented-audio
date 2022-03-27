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

protocol KnobParameterLike: ObservableObject {
    var label: String { get }
    var globalId: ParameterId { get }
    var style: KnobStyle { get }

    func formatValue() -> String
    func setKnobValue(_ value: Double)
    func toKnobValue() -> Float
    func resetDefault()
    func endParameterLock(_ store: Store)
}

extension IntParameter: KnobParameterLike {
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

    func endParameterLock(_: Store) {
        // TODO: - Implement parameter locks for slice
    }
}

extension FloatParameter: KnobParameterLike {
    func endParameterLock(_ store: Store) {
        store.endParameterLock(self)
    }

    func resetDefault() {
        setValue(defaultValue)
    }

}

struct ParameterKnobView<ParameterT: KnobParameterLike>: View {
    @ObservedObject var parameter: ParameterT
    @EnvironmentObject var store: Store

    var isDisabled: Bool = false

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
        .opacity(isDisabled ? 0.5 : 1.0)
        .allowsHitTesting(isDisabled ? false : true)
    }
}

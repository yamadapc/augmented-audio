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
import Combine

public class FloatParameter: ObservableObject, Identifiable, ParameterLike {
    public var globalId: ParameterId

    @Published var label: String
    @Published public var value: Float = 0.0

    @Published var parameterLockProgress: ParameterLockState?

    var defaultValue: Float
    var range: (Float, Float) = (0.0, 1.0)
    var style: KnobStyle = .normal

    func formatValue() -> String {
        return String(format: "%.2f", parameterLockProgress?.newValue ?? value)
    }

    init(id: ParameterId, label: String) {
        globalId = id
        self.label = label
        defaultValue = 0.0
        ALL_PARAMETERS.append(.float(self))
    }

    convenience init(id: ParameterId, label: String, style: KnobStyle, range: (Float, Float), initialValue: Float) {
        self.init(id: id, label: label)
        self.style = style
        self.range = range
        value = initialValue
        defaultValue = initialValue
    }

    convenience init(id: ParameterId, label: String, style: KnobStyle, range: (Float, Float)) {
        self.init(id: id, label: label)
        self.style = style
        self.range = range
    }

    convenience init(id: ParameterId, label: String, initialValue: Float) {
        self.init(id: id, label: label)
        value = initialValue
        defaultValue = initialValue
    }

    func setValue(_ value: Float) {
        if let parameterLockState = parameterLockProgress {
            parameterLockState.newValue = value
            objectWillChange.send()
        } else {
            self.value = value
        }
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

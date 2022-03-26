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

public class FloatParameter<LocalId>: ObservableObject, Identifiable, ParameterLike {
    public var id: LocalId
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

    init(id: LocalId, globalId: ParameterId, label: String) {
        self.id = id
        self.globalId = globalId
        self.label = label
        defaultValue = 0.0
    }

    convenience init(id: LocalId, globalId: ParameterId, label: String, style: KnobStyle, range: (Float, Float), initialValue: Float) {
        self.init(id: id, globalId: globalId, label: label)
        self.style = style
        self.range = range
        value = initialValue
        defaultValue = initialValue
    }

    convenience init(id: LocalId, globalId: ParameterId, label: String, style: KnobStyle, range: (Float, Float)) {
        self.init(id: id, globalId: globalId, label: label)
        self.style = style
        self.range = range
    }

    convenience init(id: LocalId, globalId: ParameterId, label: String, initialValue: Float) {
        self.init(id: id, globalId: globalId, label: label)
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
}

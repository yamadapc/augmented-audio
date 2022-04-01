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

var ALL_PARAMETERS: [AnyParameter] = []

protocol ParameterLike {
    var globalId: ParameterId { get }
    var label: String { get }

    var style: KnobStyle { get }
}

public enum AnyParameterInner {
    case float(FloatParameter), int(IntParameter), enumP(AnyEnumParameter), boolean(BooleanParameter)
}

extension AnyParameterInner {
    var id: ParameterId {
        switch self {
        case let .float(parameter): return parameter.globalId
        case let .int(parameter): return parameter.globalId
        case let .enumP(parameter): return parameter.globalId
        case let .boolean(parameter): return parameter.globalId
        }
    }

    func into() -> AnyParameter {
        AnyParameter(inner: self)
    }
}

/**
 * This is the same as the inner enum, but since ID is constantly read this struct caches the ID
 */
public struct AnyParameter {
    public let id: ParameterId
    let inner: AnyParameterInner

    init(inner: AnyParameterInner) {
        self.inner = inner
        id = inner.id
    }

    public func setFloatValue(_ v: Float) {
        guard case let .float(parameter) = inner else { return }
        if parameter.value != v {
            parameter.value = v
        }
    }

    public func setIntValue(_ v: Int32) {
        guard case let .int(parameter) = inner else { return }
        if parameter.value != v {
            parameter.value = Int(v)
        }
    }

    public func setBoolValue(_ v: Bool) {
        guard case let .boolean(parameter) = inner else { return }
        if parameter.value != v {
            parameter.value = v
        }
    }

    public func setEnumValue(_ v: UInt) {
        guard case var .enumP(parameter) = inner else { return }
        if parameter.rawValue != v {
            parameter.rawValue = v
        }
    }
}

public func allParameters() -> [AnyParameter] {
    return ALL_PARAMETERS
}

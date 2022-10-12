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

public protocol ParameterLike {
    var globalId: ParameterId { get }
    var label: String { get }

    var style: KnobStyle { get }
}

public enum AnyParameterInner {
    case
        float(FloatParameter),
        int(IntParameter),
        enumP(AnyEnumParameter),
        boolean(BooleanParameter)
}

extension AnyParameterInner {
    var id: ParameterId {
        get {
            switch self {
            case let .float(parameter): return parameter.globalId
            case let .int(parameter): return parameter.globalId
            case let .enumP(parameter): return parameter.globalId
            case let .boolean(parameter): return parameter.globalId
            }
        }
        set {
            switch self {
            case let .float(parameter):
                parameter.globalId = newValue
            case let .int(parameter):
                parameter.globalId = newValue
            case var .enumP(parameter):
                parameter.globalId = newValue
            case let .boolean(parameter):
                parameter.globalId = newValue
            }
        }
    }

    func into() -> AnyParameter {
        AnyParameter(inner: self)
    }

    func copy() -> AnyParameterInner {
        switch self {
        case let .float(parameter): return .float(parameter.copy())
        case let .int(parameter): return .int(parameter.copy())
        case let .enumP(parameter): return .enumP(parameter.copy())
        case let .boolean(parameter): return .boolean(parameter.copy())
        }
    }
}

/**
 * This is the same as the inner enum, but since ID is constantly read this class caches the ID
 */
public class AnyParameter: Identifiable {
    public let id: ParameterId
    var inner: AnyParameterInner

    public init(inner: AnyParameterInner) {
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

extension AnyParameter: ParameterLike {
    public var globalId: ParameterId {
        id
    }

    public var label: String {
        switch inner {
        case let .float(parameter):
            return parameter.label
        case let .int(parameter):
            return parameter.label
        case let .boolean(parameter):
            return parameter.label
        case let .enumP(parameter):
            return parameter.label
        }
    }

    public var style: KnobStyle {
        switch inner {
        case let .float(parameter):
            return parameter.style
        case let .int(parameter):
            return parameter.style
        default:
            return .normal
        }
    }
}

public func allParameters() -> [AnyParameter] {
    return ALL_PARAMETERS
}

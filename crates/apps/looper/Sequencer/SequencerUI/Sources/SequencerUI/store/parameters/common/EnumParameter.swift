import CloudKit
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

struct EnumParameterOption<OptionT> {
    let label: String
    let value: OptionT
}

public protocol AnyEnumParameter {
    var globalId: ParameterId { get set }
    var label: String { get }

    var rawValue: UInt { get set }

    func copy() -> Self
}

public protocol FromRawEnum {
    var rawValue: UInt { get }
    static func fromRaw(rawValue: UInt) -> Self
}

public class EnumParameter<OptionT: FromRawEnum>: ObservableObject, ParameterLike, AnyEnumParameter {
    public var globalId: ParameterId
    public var label: String
    public var rawValue: UInt {
        get {
            value.rawValue
        }
        set {
            value = OptionT.fromRaw(rawValue: newValue)
        }
    }

    @FastPublished public var value: OptionT
    var options: [EnumParameterOption<OptionT>]
    public var style: KnobStyle { .normal }

    required init(id: ParameterId, label: String, value: OptionT, options: [EnumParameterOption<OptionT>]) {
        globalId = id
        self.label = label
        self.value = value
        self.options = options
        ALL_PARAMETERS.append(AnyParameterInner.enumP(self).into())

        setupFastPublished(self)
    }

    public func copy() -> Self {
        return .init(id: globalId, label: label, value: value, options: options)
    }
}

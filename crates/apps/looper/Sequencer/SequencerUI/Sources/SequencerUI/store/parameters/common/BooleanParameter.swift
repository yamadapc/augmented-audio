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

public class BooleanParameter: ObservableObject, ParameterLike {
    public var globalId: ParameterId
    public var label: String
    @FastPublished public var value: Bool = false
    public var style: KnobStyle { .normal }

    init(
        id: ParameterId,
        label: String,
        value: Bool
    ) {
        globalId = id
        self.label = label
        self.value = value
        ALL_PARAMETERS.append(AnyParameterInner.boolean(self).into())

        setupFastPublished(self)
    }

    func copy() -> BooleanParameter {
        return BooleanParameter(
            id: globalId,
            label: label,
            value: value
        )
    }
}

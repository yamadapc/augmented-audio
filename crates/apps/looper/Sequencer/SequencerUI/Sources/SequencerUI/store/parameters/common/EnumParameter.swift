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

public class EnumParameter<OptionT>: ObservableObject, ParameterLike {
    var id: ParameterId
    public var globalId: ParameterId { id }
    var label: String
    @Published public var value: OptionT
    var options: [EnumParameterOption<OptionT>]
    var style: KnobStyle { .normal }

    init(id: ParameterId, label: String, value: OptionT, options: [EnumParameterOption<OptionT>]) {
        self.id = id
        self.label = label
        self.value = value
        self.options = options
    }
}

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

struct EnumParameterView<OptionT>: View
    where
    OptionT: Hashable,
    OptionT: FromRawEnum
{
    @ObservedObject var parameter: EnumParameter<OptionT>
    @EnvironmentObject var store: Store

    var body: some View {
        if #available(macOS 11.0, *) {
            Picker(parameter.label, selection: $parameter.value, content: {
                ForEach(parameter.options, id: \.value) { option in
                    Text(option.label)
                        .tag(option.value)
                }
            })
            .pickerStyle(.menu)
            .padding(PADDING - 2)
            .preferredColorScheme(.dark)
            .foregroundColor(.white)
            .accentColor(.white)
            .border(SequencerColors.blue, width: 1.0)
            .bindToParameterId(
                store: store,
                parameterId: parameter.globalId,
                showSelectionOverlay: false
            )
        } else {}
    }
}

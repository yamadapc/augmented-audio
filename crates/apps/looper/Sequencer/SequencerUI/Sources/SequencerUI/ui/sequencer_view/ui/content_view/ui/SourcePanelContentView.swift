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

struct SliceParameterKnobView: View {
    @ObservedObject var sliceEnabled: BooleanParameter
    var parameter: IntParameter<SourceParameterId>

    var body: some View {
        ParameterKnobView(
            parameter: parameter,
            isDisabled: !sliceEnabled.value
        )
    }
}

struct SourcePanelContentView: View {
    @ObservedObject var sourceParameters: SourceParametersState

    var body: some View {
        HStack(alignment: .center, spacing: 30) {
            ForEach(sourceParameters.parameters) { parameter in
                ParameterKnobView(
                    parameter: parameter
                )
            }

            ForEach(sourceParameters.intParameters) { parameter in
                if parameter.localId == .sliceId {
                    SliceParameterKnobView(
                        sliceEnabled: sourceParameters.sliceEnabled,
                        parameter: parameter
                    )
                } else {
                    ParameterKnobView(
                        parameter: parameter
                    )
                }
            }
        }
    }
}

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
    var parameters: [[Any]] {
        var output: [[Any]] = []
        var i = 0
        for parameter in sourceParameters.parameters {
            if i % 4 == 0 {
                output.append([parameter])
            } else {
                output[output.count - 1].append(parameter)
            }
            i += 1
        }
        for parameter in sourceParameters.intParameters {
            if i % 4 == 0 {
                output.append([parameter])
            } else {
                output[output.count - 1].append(parameter)
            }
            i += 1
        }
        return output
    }

    var body: some View {
        VStack(alignment: .center) {
            self.renderParameterRows()
        }
    }

    func renderParameterRows() -> some View {
        let parameters = self.parameters
        return VStack(alignment: .center, spacing: 6) {
            ForEach(0 ..< parameters.count, id: \.self) { parameterRowIndex in
                let parameterRow = parameters[parameterRowIndex]

                HStack(spacing: 30) {
                    ForEach(0 ..< parameterRow.count, id: \.self) { parameterIndex in
                        let parameter = parameterRow[parameterIndex]

                        if let p = parameter as? SourceParameter {
                            ParameterKnobView<SourceParameter>(
                                parameter: p
                            )
                        } else if let p = parameter as? IntParameter<SourceParameterId> {
                            if p.localId == .sliceId {
                                SliceParameterKnobView(
                                    sliceEnabled: sourceParameters.sliceEnabled,
                                    parameter: p
                                )
                            } else {
                                ParameterKnobView(
                                    parameter: p
                                )
                            }
                        }
                    }
                }

                if parameterRowIndex == 0 {
                    Rectangle()
                        .fill(SequencerColors.black3)
                        .frame(height: 1)
                        .frame(maxWidth: .infinity)
                }
            }
        }
    }
}

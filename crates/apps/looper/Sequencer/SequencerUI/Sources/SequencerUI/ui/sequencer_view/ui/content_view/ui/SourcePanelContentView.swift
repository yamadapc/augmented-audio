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

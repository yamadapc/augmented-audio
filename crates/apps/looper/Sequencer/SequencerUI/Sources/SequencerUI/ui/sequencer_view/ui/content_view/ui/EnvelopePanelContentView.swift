import SwiftUI

struct EnvelopePanelContentView: View {
    @ObservedObject var envelope: EnvelopeState

    var body: some View {
        HStack(alignment: .center, spacing: 30) {
            ForEach(envelope.parameters) { parameter in
                ParameterKnobView(parameter: parameter)
            }
        }
    }
}

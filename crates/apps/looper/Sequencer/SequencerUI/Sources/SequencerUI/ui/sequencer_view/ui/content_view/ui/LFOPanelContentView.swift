import SwiftUI

struct LFOPanelContentView: View {
    @EnvironmentObject var store: Store
    @ObservedObject var lfoState: LFOState

    var body: some View {
        HStack {
            KnobView(
                label: "LFO amount",
                onChanged: { value in
                    lfoState.amount = value
                },
                formatValue: { value in
                    "\(String(format: "%.0f", value * 100))%"
                },
                value: lfoState.amount
            )
            .bindToParameter(store: store, parameter: lfoState.amountParameter)

            KnobView(
                label: "LFO frequency",
                onChanged: { value in
                    lfoState.frequency = value * (20 - 0.01) + 0.01
                },
                formatValue: { value in
                    let frequency = value * (20 - 0.01) + 0.01
                    return "\(String(format: "%.2f", frequency))Hz"
                },
                value: (lfoState.frequency - 0.01) / (20 - 0.01)
            )
            .bindToParameter(store: store, parameter: lfoState.frequencyParameter)
        }
    }
}

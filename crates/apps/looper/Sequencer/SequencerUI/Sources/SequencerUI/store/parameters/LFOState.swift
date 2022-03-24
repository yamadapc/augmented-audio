import Combine

public enum LFOParameterId {
    case frequency, amount
}

typealias LFOParameter = FloatParameter<LFOParameterId>

class LFOState: ObservableObject, LFOVisualisationViewModel {
    var trackId: Int
    var index: UInt

    var frequency: Double {
        get {
            Double(frequencyParameter.value)
        }
        set {
            frequencyParameter.value = Float(newValue)
            objectWillChange.send()
        }
    }

    var amount: Double {
        get {
            Double(amountParameter.value)
        }
        set {
            amountParameter.value = Float(newValue)
            objectWillChange.send()
        }
    }

    @Published var frequencyParameter: LFOParameter
    @Published var amountParameter: LFOParameter

    init(trackId: Int, index: UInt) {
        self.trackId = trackId
        self.index = index

        frequencyParameter = .init(
            id: .frequency,
            globalId: .lfoParameter(trackId: trackId, lfo: index, parameterId: .frequency),
            label: "Frequency",
            initialValue: 1.0
        )
        amountParameter = .init(
            id: .amount,
            globalId: .lfoParameter(trackId: trackId, lfo: index, parameterId: .amount),
            label: "Amount",
            initialValue: 1.0
        )
    }
}

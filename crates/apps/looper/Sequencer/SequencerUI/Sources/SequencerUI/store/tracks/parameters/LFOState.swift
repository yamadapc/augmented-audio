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

public enum LFOParameterId: Equatable {
    case frequency, amount
}

public class LFOAmountParameter: FloatParameter<LFOParameterId> {
    override func formatValue() -> String {
        return "\(String(format: "%.0f", value * 100))%"
    }
}

public class LFOFrequencyParameter: FloatParameter<LFOParameterId> {
    override func formatValue() -> String {
        let frequency = value * (20 - 0.01) + 0.01
        return "\(String(format: "%.2f", frequency))Hz"
    }
}

public class LFOState: ObservableObject, LFOVisualisationViewModel {
    var trackId: Int
    var index: UInt

    var label: String {
        "LFO \(index + 1)"
    }

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

    @Published var frequencyParameter: LFOFrequencyParameter
    @Published var amountParameter: LFOAmountParameter

    public var parameters: [FloatParameter<LFOParameterId>] { [
        frequencyParameter,
        amountParameter,
    ] }

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

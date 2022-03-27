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

extension LFOParameterId {
    func shortHelpString() -> String {
        switch self {
        case .frequency:
            return "LFO Frequency"
        case .amount:
            return "LFO Amount"
        }
    }
}

public class LFOAmountParameter: FloatParameter<LFOParameterId> {
    override func formatValue() -> String {
        return "\(String(format: "%.0f", value * 100))%"
    }
}

public class LFOFrequencyParameter: FloatParameter<LFOParameterId> {
    override func formatValue() -> String {
        let frequency = value
        return "\(String(format: "%.2f", frequency))Hz"
    }

    override func toKnobValue() -> Float {
        return (value - 0.01) / (20.0 - 0.01)
    }

    override func fromKnobValue(knobValue: Double) -> Float {
        return Float(knobValue * (20.0 - 0.01) + 0.01)
    }

}

public class LFOState: ObservableObject {
    var trackId: Int
    var index: UInt

    var label: String {
        "LFO \(index + 1)"
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

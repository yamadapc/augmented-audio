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

public enum QuantizationMode: UInt {
    case snapNext, snapClosest, none
}

extension QuantizationMode: FromRawEnum {
    public static func fromRaw(rawValue: UInt) -> Self {
        QuantizationMode(rawValue: rawValue)!
    }
}

public enum TempoControlMode: UInt {
    case setAndFollowGlobalTempo, none
}

extension TempoControlMode: FromRawEnum {
    public static func fromRaw(rawValue: UInt) -> Self {
        TempoControlMode(rawValue: rawValue)!
    }
}

public class QuantizationParameters: ObservableObject {
    public var quantizationMode: EnumParameter<QuantizationMode>
    public var tempoControlMode: EnumParameter<TempoControlMode>

    init(trackId: UInt) {
        quantizationMode = EnumParameter(
            id: .quantizationMode(trackId: trackId),
            label: "Quantization",
            value: .snapNext,
            options: [
                EnumParameterOption(label: "Snap next", value: .snapNext),
                EnumParameterOption(label: "Snap closest", value: .snapClosest),
                EnumParameterOption(label: "None", value: .none),
            ]
        )
        tempoControlMode = EnumParameter(
            id: .tempoControl(trackId: trackId),
            label: "Tempo ctrl.",
            value: .setAndFollowGlobalTempo,
            options: [
                EnumParameterOption(label: "Set global tempo", value: .setAndFollowGlobalTempo),
                EnumParameterOption(label: "None", value: .none),
            ]
        )
    }
}

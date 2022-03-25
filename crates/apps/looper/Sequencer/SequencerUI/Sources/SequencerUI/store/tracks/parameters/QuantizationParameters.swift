import Combine

public enum QuantizationMode {
    case snapNext, snapClosest, none
}

public enum TempoControlMode {
    case setAndFollowGlobalTempo, none
}

public class QuantizationParameters: ObservableObject {
    public var quantizationMode: EnumParameter<QuantizationMode>
    public var tempoControlMode: EnumParameter<TempoControlMode>

    init(trackId: Int) {
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

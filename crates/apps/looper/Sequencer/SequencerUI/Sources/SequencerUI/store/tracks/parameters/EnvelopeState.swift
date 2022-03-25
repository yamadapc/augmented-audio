import Combine

public enum EnvelopeParameterId {
    case attack, decay, release, sustain, enabled
}

public typealias EnvelopeParameter = FloatParameter<EnvelopeParameterId>

public class EnvelopeState: ObservableObject {
    var trackId: Int
    @Published var enabled: BooleanParameter
    @Published var attack: EnvelopeParameter
    @Published var decay: EnvelopeParameter
    @Published var sustain: EnvelopeParameter
    @Published var release: EnvelopeParameter

    public var parameters: [EnvelopeParameter] {
        [
            attack,
            decay,
            sustain,
            release,
        ]
    }

    public var toggles: [BooleanParameter] {
        [
            enabled,
        ]
    }

    var cancellables: Set<AnyCancellable> = Set()

    init(trackId: Int) {
        self.trackId = trackId
        enabled = .init(
            id: .envelopeParameter(trackId: trackId, parameterId: .enabled),
            label: "Envelope enabled",
            value: false
        )
        attack = .init(
            id: .attack,
            globalId: .envelopeParameter(trackId: trackId, parameterId: .attack),
            label: "Attack",
            initialValue: 0
        )
        decay = .init(
            id: .decay,
            globalId: .envelopeParameter(trackId: trackId, parameterId: .decay),
            label: "Decay",
            initialValue: 0.2
        )
        sustain = .init(
            id: .sustain,
            globalId: .envelopeParameter(trackId: trackId, parameterId: .sustain),
            label: "Sustain",
            initialValue: 0.8
        )
        release = .init(
            id: .release,
            globalId: .envelopeParameter(trackId: trackId, parameterId: .release),
            label: "Release",
            initialValue: 0.3
        )

        parameters.forEach { parameter in
            parameter.$value.sink { _ in
                self.objectWillChange.send()
            }.store(in: &cancellables)
        }
    }
}

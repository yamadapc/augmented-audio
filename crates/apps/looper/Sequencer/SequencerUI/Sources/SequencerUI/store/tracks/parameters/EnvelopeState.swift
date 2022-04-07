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

public enum EnvelopeParameterId {
    case attack, decay, release, sustain, enabled
}

extension EnvelopeParameterId {
    func shortHelpString() -> String {
        switch self {
        case .attack:
            return "Attack"
        case .decay:
            return "Decay"
        case .release:
            return "Release"
        case .sustain:
            return "Sustain"
        case .enabled:
            return "Enabled - If enabled, the envelope will be applied to the loop on each trigger"
        }
    }
}

public typealias EnvelopeParameter = FloatParameter

public class EnvelopeState: ObservableObject {
    var trackId: UInt
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

    init(trackId: UInt) {
        self.trackId = trackId
        enabled = .init(
            id: .envelopeParameter(trackId: trackId, parameterId: .enabled),
            label: "Envelope enabled",
            value: false
        )
        attack = .init(
            id: .envelopeParameter(trackId: trackId, parameterId: .attack),
            label: "Attack",
            initialValue: 0
        )
        decay = .init(
            id: .envelopeParameter(trackId: trackId, parameterId: .decay),
            label: "Decay",
            initialValue: 0.2
        )
        sustain = .init(
            id: .envelopeParameter(trackId: trackId, parameterId: .sustain),
            label: "Sustain",
            initialValue: 0.8
        )
        release = .init(
            id: .envelopeParameter(trackId: trackId, parameterId: .release),
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

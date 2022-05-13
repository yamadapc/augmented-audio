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
import Logging
import SequencerEngine_private
import SequencerUI

/**
 * This class pushes data from Swift reactive objects into the audio engine.
 */
class StoreSubscriptionsController {
    private let store: Store
    private let engine: EngineImpl
    private var cancellables: Set<AnyCancellable> = Set()
    private let logger = Logger(label: "com.beijaflor.sequencer.engine.StoreSubscriptionsController")

    init(store: Store, engine: EngineImpl) {
        self.store = store
        self.engine = engine
    }

    func setup() {
        store.trackStates.enumerated().forEach { looperId, trackState in
            setupTrack(UInt(looperId), trackState)
        }

        store.sceneState.sceneSlider.$value.sink(receiveValue: { value in
            self.logger.debug("Setting scene", metadata: [
                "value": .stringConvertible(value),
            ])
            self.engine.setSceneSliderValue(value: (value + 1.0) / 2.0)
        }).store(in: &cancellables)

        store.metronomeVolume.$value.sink(receiveValue: { value in
            self.engine.setMetronomeVolume(volume: value)
        }).store(in: &cancellables)

        store.$selectedTrack.sink(receiveValue: { value in
            self.engine.setActiveLooper(looperId: UInt(value))
        }).store(in: &cancellables)
    }

    /**
     * Given a Float `Publisher` value `publisher`, an `initialValue` and a `flush` function,
     * create a subscription which will call flush on every new value and call `flush` with initial value.
     */
    func pushFloatValue<PublisherT: Publisher>(
        publisher: PublisherT,
        flush: @escaping (_ value: Float) -> Void,
        initialValue: Float?
    )
        where
        PublisherT.Output == Float,
        PublisherT.Failure == Never
    {
        if let initialValue = initialValue {
            flush(initialValue)
        }
        publisher.sink(receiveValue: { value in flush(value) }).store(in: &cancellables)
    }

    private func setupTrack(_ looperId: UInt, _ trackState: TrackState) {
        pushFloatValue(
            publisher: trackState.volumeParameter.$value,
            flush: { self.engine.setVolume(looperId, volume: $0) },
            initialValue: trackState.volumeParameter.value
        )

        setupSourceParameters(looperId: looperId, sourceParameters: trackState.sourceParameters)
        setupLFOSync(looperId: looperId, lfoId: 0, lfoState: trackState.lfo1)
        setupLFOSync(looperId: looperId, lfoId: 1, lfoState: trackState.lfo2)
        setupEnvelopeParameters(looperId: looperId, envelope: trackState.envelope)
        setupQuantizationParameters(looperId: looperId, quantizationParameters: trackState.quantizationParameters)
    }

    private func setupSourceParameters(looperId: UInt, sourceParameters: SourceParametersState) {
        sourceParameters.parameters.forEach { parameter in
            let flush = { self.flushFloatSourceParameter(looperId: looperId, parameter: parameter, value: $0) }
            pushFloatValue(
                publisher: parameter.$value,
                flush: { flush($0) },
                initialValue: parameter.value
            )
        }

        sourceParameters.intParameters.forEach { parameter in
            let flush = { self.flushIntSourceParameter(looperId: looperId, parameter: parameter, value: $0) }

            flush(parameter.value)
            parameter.$value.sink(receiveValue: { flush($0) }).store(in: &cancellables)
        }

        setupToggles(looperId: looperId, toggles: sourceParameters.toggles)
    }

    private func setupLFOSync(looperId: UInt, lfoId: UInt, lfoState: LFOState) {
        lfoState.parameters.forEach { parameter in
            pushFloatValue(
                publisher: parameter.$value,
                flush: {
                    if case let .lfoParameter(_, _, parameter) = parameter.globalId {
                        self.engine.setLFOParameter(
                            looperId,
                            parameterId: parameter,
                            lfoId: lfoId,
                            value: $0
                        )
                    }
                },
                initialValue: parameter.value
            )
        }

        lfoState.modeParameter.$value.sink(receiveValue: { value in
            guard let rustValue = LFO_MODES[value] else { return }
            self.engine.setLFOMode(looperId: looperId, lfoId: lfoId, value: rustValue)
        }).store(in: &cancellables)
    }

    private func setupEnvelopeParameters(looperId: UInt, envelope: EnvelopeState) {
        envelope.parameters.forEach { parameter in
            guard case let .envelopeParameter(_, parameterId) = parameter.globalId
            else { return }

            pushFloatValue(
                publisher: parameter.$value,
                flush: {
                    self.engine.setEnvelopeParameter(
                        looperId: looperId,
                        parameterId: ENVELOPE_PARAMETER_IDS[parameterId]!,
                        value: $0
                    )
                }, initialValue: parameter.value
            )
        }

        setupToggles(looperId: looperId, toggles: envelope.toggles)
    }

    private func setupQuantizationParameters(looperId: UInt, quantizationParameters: QuantizationParameters) {
        quantizationParameters.quantizationMode.$value.sink(receiveValue: { value in
            self.engine.setQuantizationMode(looperId: looperId, mode: RUST_QUANTIZE_MODES[value]!)
        }).store(in: &cancellables)

        quantizationParameters.tempoControlMode.$value.sink(receiveValue: { value in
            self.engine.setTempoControl(looperId: looperId, tempoControl: RUST_TEMPO_CONTROL[value]!)
        }).store(in: &cancellables)
    }

    private func setupToggles(looperId: UInt, toggles: [BooleanParameter]) {
        toggles.forEach { toggle in
            let flush = { self.engine.setBooleanParameter(looperId, parameterId: toggle.globalId, value: $0) }
            flush(toggle.value)
            toggle.$value.sink(receiveValue: { value in flush(value) }).store(in: &cancellables)
        }
    }

    private func flushFloatSourceParameter(looperId: UInt, parameter: FloatParameter, value: Float) {
        logger.debug("Setting source parameter", metadata: [
            "id": .string(String(describing: parameter.globalId)),
            "value": .stringConvertible(value),
        ])
        if case let .sourceParameter(_, parameter) = parameter.globalId {
            self.engine.setSourceParameter(
                looperId,
                parameterId: parameter,
                value: value
            )
        }
    }

    private func flushIntSourceParameter(looperId: UInt, parameter: IntParameter, value: Int) {
        guard case let .sourceParameter(_, parameterId) = parameter.globalId
        else { return }
        engine.setSourceParameterInt(
            looperId,
            parameterId: parameterId,
            value: Int32(value)
        )
    }
}

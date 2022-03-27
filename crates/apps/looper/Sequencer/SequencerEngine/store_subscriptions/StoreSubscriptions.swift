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
            looper_engine__set_scene_slider_value(self.engine.engine, (value + 1.0) / 2.0)
        }).store(in: &cancellables)

        store.metronomeVolume.$value.sink(receiveValue: { value in
            looper_engine__set_metronome_volume(self.engine.engine, value)
        }).store(in: &cancellables)
    }

    func pushFloatValue<PublisherT: Publisher>(publisher: PublisherT, flush: @escaping (_ value: Float) -> Void, initialValue: Float?)
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

        trackState.sourceParameters.parameters.forEach { parameter in
            pushFloatValue(
                publisher: parameter.$value,
                flush: {
                    self.logger.debug("Setting source parameter", metadata: [
                        "id": .stringConvertible(parameter.id.debugDescription),
                        "value": .stringConvertible($0),
                    ])
                    self.engine.setSourceParameter(looperId, parameterId: parameter.id, value: $0)
                },
                initialValue: parameter.value
            )
        }

        trackState.sourceParameters.intParameters.forEach { parameter in
            let flush = { (value: Int) in
                self.engine.setSourceParameterInt(
                    looperId,
                    parameterId: parameter.localId,
                    value: Int32(value)
                )
            }
            parameter.$value.sink(receiveValue: { value in flush(value) }).store(in: &cancellables)
            flush(parameter.value)
        }

        trackState.sourceParameters.toggles.forEach { toggle in
            let flush = { (value: Bool) in
                self.engine.setBooleanParameter(looperId, parameterId: toggle.id, value: value)
            }
            toggle.$value.sink(receiveValue: { value in flush(value) }).store(in: &cancellables)
            flush(toggle.value)
        }

        trackState.lfo1.parameters.forEach { parameter in
            pushFloatValue(
                publisher: parameter.$value,
                flush: {
                    self.engine.setLFOParameter(
                        looperId,
                        parameterId: parameter.id,
                        lfoId: 0,
                        value: $0
                    )
                },
                initialValue: parameter.value
            )
        }
        trackState.lfo2.parameters.forEach { parameter in
            pushFloatValue(
                publisher: parameter.$value,
                flush: {
                    self.engine.setLFOParameter(
                        looperId,
                        parameterId: parameter.id,
                        lfoId: 1,
                        value: $0
                    )
                },
                initialValue: parameter.value
            )
        }

        trackState.envelope.parameters.forEach { parameter in
            let rustParameterId = ENVELOPE_PARAMETER_IDS[parameter.id]!

            pushFloatValue(
                publisher: parameter.$value,
                flush: {
                    looper_engine__set_envelope_parameter(
                        self.engine.engine,
                        looperId,
                        rustParameterId,
                        $0
                    )
                }, initialValue: parameter.value
            )
        }
        trackState.envelope.toggles.forEach { toggle in
            let rustParameterId = getObjectIdRust(toggle.id)!
            let flush = { (value: Bool) in
                looper_engine__set_boolean_parameter(
                    self.engine.engine,
                    looperId,
                    rustParameterId,
                    value
                )
            }
            toggle.$value.sink(receiveValue: { value in flush(value) }).store(in: &cancellables)
            flush(toggle.value)
        }

        trackState.quantizationParameters.quantizationMode.$value.sink(receiveValue: { value in
            looper_engine__set_quantization_mode(self.engine.engine, looperId, RUST_QUANTIZE_MODES[value]!)
        }).store(in: &cancellables)

        trackState.quantizationParameters.tempoControlMode.$value.sink(receiveValue: { value in
            looper_engine__set_tempo_control(self.engine.engine, looperId, RUST_TEMPO_CONTROL[value]!)
        }).store(in: &cancellables)
    }
}

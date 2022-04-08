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

import Logging
import SequencerEngine_private
import SequencerUI

class EffectsServiceImpl: EffectsService {
    private let logger = Logger(label: "com.beijaflor.sequencer.engine.EngineController")

    func listEffects() -> [EffectDefinition] {
        logger.info("Listing effect definitions")

        let definitions = looper_engine__get_effect_definitions()
        let numDefinitions = effect_definitions__count(definitions)

        var effects: [EffectDefinition] = []
        for definitionIndex in 0 ..< numDefinitions {
            let definition = effect_definitions__get(definitions, definitionIndex)
            let name = effect_definition__name(definition)
            let nameStr = String(cString: name!)
            c_string_free(name)

            let parameters = getParameters(definition: definition!)
            let effect = buildEffect(
                id: definitionIndex,
                label: nameStr,
                parameters: parameters
            )
            effects.append(effect)
        }
        effect_definitions__free(definitions)

        return effects
    }

    func getParameters(definition: OpaquePointer) -> [AnyParameter] {
        let parametersList = effect_definition__parameters(definition)
        let numParameters = effect_parameters__count(parametersList)

        var result: [AnyParameter] = []
        for parameterIndex in 0 ..< numParameters {
            let parameter = effect_parameters__get(parametersList, parameterIndex)
            let parameterName = effect_parameter__label(parameter)
            let parameterNameStr = String(cString: parameterName!)
            c_string_free(parameterName)

            logger.info("Loaded effect", metadata: ["parameterName": .string(parameterNameStr)])
            let anyParameter = AnyParameter(
                inner: .float(FloatParameter(
                    id: .effectsParameter(trackId: 0, slotId: 0, parameterId: parameterIndex),
                    label: parameterNameStr
                ))
            )

            result.append(anyParameter)
        }

        effect_parameters__free(parametersList)
        return result
    }
}

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
import SwiftUI

public enum ParameterLockSource: Hashable {
    case stepId(StepId), sceneId(SceneId), lfoId(LFOId)
}

extension ParameterLockSource {
    func toParameterId() -> ParameterId {
        switch self {
        case let .stepId(stepId):
            return ParameterId.stepButton(trackId: stepId.trackId, stepId: stepId.stepIndex)
        case let .sceneId(sceneId):
            return ParameterId.sceneButton(sceneId: sceneId.index)
        case let .lfoId(lfoId):
            return ParameterId.lfo(trackId: lfoId.trackId, lfoIndex: Int(lfoId.index))
        }
    }
}

public struct ParameterLockId: Hashable {
    public let parameterId: ParameterId
    public let source: ParameterLockSource
}

class ParameterLockState: ObservableObject, Identifiable {
    var id: ParameterLockId {
        ParameterLockId(parameterId: parameterId, source: source)
    }

    let parameterId: ParameterId
    let source: ParameterLockSource
    let color: Color

    @Published var newValue: Float?

    init(parameterId: ParameterId, source: ParameterLockSource) {
        self.parameterId = parameterId
        self.source = source
        color = SequencerColors.randomColor(ParameterLockId(parameterId: parameterId, source: source))
    }
}

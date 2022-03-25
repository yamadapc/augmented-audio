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

class SceneModel: ObservableObject {
    @Published var parameterLocks: [ObjectId: ParameterLockState] = [:]

    init() {}
}

public class SceneState: ObservableObject {
    @Published public var sceneSlider = FloatParameter(
        id: 0,
        globalId: .sceneSlider,
        label: "Scene slider",
        style: .center,
        range: (-1.0, 1.0),
        initialValue: -1.0
    )
    @Published var scenes: [SceneModel] = [
        SceneModel(),
        SceneModel(),
    ]

    init() {}
}

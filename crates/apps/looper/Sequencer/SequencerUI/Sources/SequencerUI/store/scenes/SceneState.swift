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

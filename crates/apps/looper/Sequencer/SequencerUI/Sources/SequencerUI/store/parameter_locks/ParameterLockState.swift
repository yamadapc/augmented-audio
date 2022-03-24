import Combine

enum ParameterLockSource {
    case stepId(Int), sceneId(Int)
}

class ParameterLockState: ObservableObject {
    let parameterId: ObjectId
    let source: ParameterLockSource

    @Published var newValue: Float?

    init(parameterId: ObjectId, source: ParameterLockSource) {
        self.parameterId = parameterId
        self.source = source
    }
}


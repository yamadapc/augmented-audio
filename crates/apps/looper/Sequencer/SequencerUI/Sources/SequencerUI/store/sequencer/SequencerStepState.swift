import Combine

public class SequencerStepState: ObservableObject {
    var index: Int
    @Published var parameterLocks: [ParameterLockState] = []

    init(index: Int) {
        self.index = index
    }
}

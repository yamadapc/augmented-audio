import Combine

public class IntParameter<ParameterId>: ObservableObject, Identifiable {
    public var id: ObjectId
    public var localId: ParameterId
    @Published var label: String
    @Published public var value: Int
    @Published var maximum: Int

    init(id: ObjectId, localId: ParameterId, label: String, value: Int, maximum _: Int) {
        self.id = id
        self.localId = localId
        self.label = label
        self.value = value
        maximum = 0
    }
}

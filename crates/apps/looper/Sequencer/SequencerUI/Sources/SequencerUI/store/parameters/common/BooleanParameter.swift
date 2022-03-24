import Combine

public class BooleanParameter: ObservableObject {
    public var id: ObjectId
    var label: String
    @Published public var value: Bool = false

    init(
        id: ObjectId,
        label: String,
        value: Bool
    ) {
        self.id = id
        self.label = label
        self.value = value
    }
}

import Combine

struct EnumParameterOption<OptionT> {
    let label: String
    let value: OptionT
}

public class EnumParameter<OptionT>: ObservableObject {
    var id: ObjectId
    var label: String
    @Published public var value: OptionT
    var options: [EnumParameterOption<OptionT>]

    init(id: ObjectId, label: String, value: OptionT, options: [EnumParameterOption<OptionT>]) {
        self.id = id
        self.label = label
        self.value = value
        self.options = options
    }
}


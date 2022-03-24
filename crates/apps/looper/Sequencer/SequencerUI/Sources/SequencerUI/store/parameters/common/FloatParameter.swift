import Combine

public class FloatParameter<ParameterId>: ObservableObject, Identifiable {
    public var id: ParameterId
    var globalId: ObjectId

    @Published var label: String
    @Published public var value: Float = 0.0

    @Published var parameterLockProgress: ParameterLockState?

    var defaultValue: Float
    var range: (Float, Float) = (0.0, 1.0)
    var style: KnobStyle = .normal

    init(id: ParameterId, globalId: ObjectId, label: String) {
        self.id = id
        self.globalId = globalId
        self.label = label
        defaultValue = 0.0
    }

    convenience init(id: ParameterId, globalId: ObjectId, label: String, style: KnobStyle, range: (Float, Float), initialValue: Float) {
        self.init(id: id, globalId: globalId, label: label)
        self.style = style
        self.range = range
        value = initialValue
        defaultValue = initialValue
    }

    convenience init(id: ParameterId, globalId: ObjectId, label: String, style: KnobStyle, range: (Float, Float)) {
        self.init(id: id, globalId: globalId, label: label)
        self.style = style
        self.range = range
    }

    convenience init(id: ParameterId, globalId: ObjectId, label: String, initialValue: Float) {
        self.init(id: id, globalId: globalId, label: label)
        value = initialValue
        defaultValue = initialValue
    }

    func setValue(_ value: Float) {
        if let parameterLockState = parameterLockProgress {
            parameterLockState.newValue = value
            objectWillChange.send()
        } else {
            self.value = value
        }
    }
}

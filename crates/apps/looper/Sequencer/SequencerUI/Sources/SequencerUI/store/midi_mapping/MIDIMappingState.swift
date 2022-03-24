import Combine

enum MIDIMessageId: Hashable {
    case cc(Int)
}

extension MIDIMessageId {
    func toString() -> String {
        switch self {
        case let .cc(controllerNumber):
            return "CC \(controllerNumber)"
        }
    }
}

public struct MIDIMessage: Hashable {
    let controllerNumber: Int
    let value: Int

    public init(controllerNumber: Int, value: Int) {
        self.controllerNumber = controllerNumber
        self.value = value
    }
}

public class MIDIMappingState: ObservableObject {
    var midiMap: [MIDIMessageId: ObjectId] = [:]
    var lastMidiMessages: [(Int, MIDIMessage)] = []
    var lastMessagesMap: [Int: MIDIMessage] = [:]

    var mapKeys: [MIDIMessageId] {
        Array(midiMap.keys)
    }

    var currentMessageId = 0

    func addMapping(id: MIDIMessageId, objectId: ObjectId) {
        midiMap[id] = objectId
        objectWillChange.send()
    }

    func addMidiMessage(message: MIDIMessage) {
        if lastMessagesMap[message.controllerNumber]?.value == message.value {
            return
        }

        currentMessageId += 1
        lastMessagesMap[message.controllerNumber] = message
        lastMidiMessages.append((currentMessageId, message))
        let newLength = min(
            lastMidiMessages.count,
            100
        )
        lastMidiMessages.removeFirst(lastMidiMessages.count - newLength)
        objectWillChange.send()
    }
}


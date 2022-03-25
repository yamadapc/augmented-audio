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
    var midiMap: [MIDIMessageId: ParameterId] = [:]
    var lastMidiMessages: [(Int, MIDIMessage)] = []
    var lastMessagesMap: [Int: MIDIMessage] = [:]

    var mapKeys: [MIDIMessageId] {
        Array(midiMap.keys)
    }

    var currentMessageId = 0

    func addMapping(id: MIDIMessageId, objectId: ParameterId) {
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

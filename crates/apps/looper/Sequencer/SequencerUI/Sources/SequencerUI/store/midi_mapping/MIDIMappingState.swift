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

/**
 * When a MIDI message is received by the rust side, it's pushed onto the UI.
 * This object holds the last few messages & the current CC values received.
 *
 * The purpouse is to display a monitor of last messages/values. Message events are tagged with an internal ID,
 * which is just an incrementing counter.
 *
 * This object also holds the current MIDI map.
 */
public class MIDIMappingState: ObservableObject {
    private var midiMap: [MIDIMessageId: ParameterId] = [:]
    private var lastMidiMessages: [(MIDIMessageIndex, MIDIMessage)] = []
    private var lastMessagesMap: [MIDIControllerNumber: MIDIMessage] = [:]

    var mapKeys: [MIDIMessageId] {
        Array(midiMap.keys)
    }

    var currentMessageId = 0

    func getLastMidiMessages() -> [(MIDIMessageIndex, MIDIMessage)] {
        return self.lastMidiMessages
    }

    func getMapping(message: MIDIMessageId) -> ParameterId? {
        return midiMap[message]
    }

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
        lastMidiMessages.append((MIDIMessageIndex(raw: currentMessageId), message))
        let newLength = min(
            lastMidiMessages.count,
            100
        )
        lastMidiMessages.removeFirst(lastMidiMessages.count - newLength)
        objectWillChange.send()
    }

    struct MIDIMessageIndex: Hashable {
        let raw: Int
    }
}

public struct MIDIControllerNumber: Hashable {
    let raw: Int

    public init(raw: Int) {
        self.raw = raw
    }

    public init(_ raw: Int) {
        self.init(raw: raw)
    }
}

enum MIDIMessageId: Hashable {
    case cc(MIDIControllerNumber)
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
    let controllerNumber: MIDIControllerNumber
    let value: Int

    public init(controllerNumber: MIDIControllerNumber, value: Int) {
        self.controllerNumber = controllerNumber
        self.value = value
    }

    public init(controllerNumber: Int, value: Int) {
        self.controllerNumber = MIDIControllerNumber(controllerNumber)
        self.value = value
    }
}

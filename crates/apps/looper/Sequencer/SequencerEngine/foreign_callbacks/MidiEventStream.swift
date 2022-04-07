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
import Foundation
import Logging
import SequencerEngine_private

@_cdecl("swift__midi_callback")
private func midi_callback(userdata: UnsafeMutableRawPointer?, id: MidiEvent) {
    let wrappedClosure: MidiEventWrapClosure = Unmanaged.fromOpaque(userdata!).takeUnretainedValue()
    wrappedClosure.closure(id)
}

private class MidiEventWrapClosure {
    let closure: (MidiEvent) -> Void

    private init(closure: @escaping (MidiEvent) -> Void) {
        self.closure = closure
    }

    fileprivate static func build(closure: @escaping (MidiEvent) -> Void) -> ForeignCallback_MidiEvent {
        let wrappedClosure = MidiEventWrapClosure(closure: closure)
        let context = Unmanaged.passRetained(wrappedClosure).toOpaque()
        return ForeignCallback_MidiEvent(
            context: context,
            callback: midi_callback
        )
    }
}

func buildStream(
    registerStream: @escaping (ForeignCallback_MidiEvent) -> Void
) -> AnyPublisher<MidiEvent, Never> {
    let publisher = PassthroughSubject<MidiEvent, Never>()

    let foreignCallback = MidiEventWrapClosure.build { id in
        publisher.send(id)
    }
    registerStream(foreignCallback)

    return AnyPublisher(publisher)
}

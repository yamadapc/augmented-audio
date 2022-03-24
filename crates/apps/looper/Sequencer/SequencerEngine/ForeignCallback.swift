//
//  RustStream.swift
//  SequencerEngine
//
//  Created by Pedro Tacla Yamada on 24/3/2022.
//

import Combine
import Foundation
import Logging
import SequencerEngine_private

@_cdecl("swift__wrap_closure_callback")
private func midi_callback(userdata: UnsafeMutableRawPointer?, id: MidiEvent) {
    let wrappedClosure: MidiEventWrapClosure = Unmanaged.fromOpaque(userdata!).takeUnretainedValue()
    wrappedClosure.closure(id)
}

private class MidiEventWrapClosure {
    let closure: (MidiEvent) -> Void

    private init(closure: @escaping (MidiEvent) -> Void) {
        self.closure = closure
    }

    fileprivate static func build(closure: @escaping (MidiEvent) -> Void) -> ForeignCallback_MidiEvent
    {
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

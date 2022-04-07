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

@_cdecl("swift__application_event_callback")
private func application_event_callback(userdata: UnsafeMutableRawPointer?, id: ApplicationEvent) {
    let wrappedClosure: ApplicationEventWrapClosure = Unmanaged.fromOpaque(userdata!).takeUnretainedValue()
    wrappedClosure.closure(id)
}

private class ApplicationEventWrapClosure {
    let closure: (ApplicationEvent) -> Void

    private init(closure: @escaping (ApplicationEvent) -> Void) {
        self.closure = closure
    }

    fileprivate static func build(closure: @escaping (ApplicationEvent) -> Void) -> ForeignCallback_ApplicationEvent {
        let wrappedClosure = ApplicationEventWrapClosure(closure: closure)
        let context = Unmanaged.passRetained(wrappedClosure).toOpaque()
        return ForeignCallback_ApplicationEvent(
            context: context,
            callback: application_event_callback
        )
    }
}

func buildApplicationEventStream(
    registerStream: @escaping (ForeignCallback_ApplicationEvent) -> Void
) -> AnyPublisher<ApplicationEvent, Never> {
    let publisher = PassthroughSubject<ApplicationEvent, Never>()

    let foreignCallback = ApplicationEventWrapClosure.build { id in
        publisher.send(id)
    }
    registerStream(foreignCallback)

    return AnyPublisher(publisher)
}

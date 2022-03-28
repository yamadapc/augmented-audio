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
import SwiftUI

private func appendInsert<K, V>(dict: inout [K: [V]], key: K, value: V) {
    if dict[key] == nil {
        dict[key] = []
    }

    dict[key]!.append(value)
}

class ParameterLockStore: ObservableObject {
    private var locks: [ParameterLockSource: [ParameterId: ParameterLockState]] = [:]
    private var allLocks: [ParameterId: [ParameterLockState]] = [:]

    init() {}

    func addLock(lock: ParameterLockState) {
        if locks[lock.source] == nil {
            locks[lock.source] = [:]
        }
        locks[lock.source]![lock.parameterId] = lock

        appendInsert(dict: &allLocks, key: lock.parameterId, value: lock)
        appendInsert(dict: &allLocks, key: lock.source.toParameterId(), value: lock)

        DispatchQueue.main.async {
            self.objectWillChange.send()
        }
    }

    func removeLock(_ id: ParameterLockId) {
        locks[id.source]?.removeValue(forKey: id.parameterId)
        allLocks[id.source.toParameterId()]?.removeAll(where: { $0.parameterId == id.parameterId })
        allLocks[id.parameterId]?.removeAll(where: { $0.source == id.source })

        DispatchQueue.main.async {
            self.objectWillChange.send()
        }
    }

    func hasLocks(source: ParameterLockSource) -> Bool {
        return locks[source] != nil
    }

    func getLocks(parameterId: ParameterId) -> [ParameterLockState] {
        return allLocks[parameterId] ?? []
    }
}

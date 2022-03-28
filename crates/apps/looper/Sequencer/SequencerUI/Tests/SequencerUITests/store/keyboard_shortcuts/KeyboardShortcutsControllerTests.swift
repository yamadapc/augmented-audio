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
@testable import SequencerUI
import XCTest

class KeyboardShortcutsControllerTests: XCTestCase {
    func testSpaceTriggersPlay() {
        let store = Store(engine: nil)
        let controller = KeyboardShortcutsController(store: store)
        XCTAssertEqual(store.isPlaying, false)
        controller.onKeyDown(key: .space, modifiers: [])
        XCTAssertEqual(store.isPlaying, true)
        controller.onKeyDown(key: .space, modifiers: [])
        XCTAssertEqual(store.isPlaying, false)
    }

    func testDeleteIsNoopIsTheresNoLock() {
        let store = Store(engine: nil)
        let controller = KeyboardShortcutsController(store: store)
        controller.onKeyDown(key: .delete, modifiers: [])
    }

    func testDeleteTriggersRemovalOfTheCurrentlySelectedLock() {
        let store = Store(engine: nil)
        let controller = KeyboardShortcutsController(store: store)
        let parameterId: ParameterId = .sourceParameter(
            trackId: 0,
            parameterId: .start
        )
        let sourceId: ParameterLockSource = .stepId(StepId(trackId: 0, stepIndex: 2))
        store.parameterLockStore.addLock(
            lock: .init(
                parameterId: parameterId,
                source: sourceId
            )
        )
        XCTAssertTrue(store.parameterLockStore.hasLocks(source: sourceId))
        store.focusState.selectedObject = .parameterLock(source: sourceId, parameterId: parameterId)
        controller.onKeyDown(key: .delete, modifiers: [])

        XCTAssertFalse(store.parameterLockStore.hasLocks(source: sourceId))
        XCTAssertNil(store.focusState.selectedObject)
    }
}

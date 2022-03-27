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
import SnapshotTesting
import SwiftUI
import ViewInspector
import XCTest

final class SequencerUITests: XCTestCase {
    func testCreateStore() {
        let store = Store(engine: nil)
        XCTAssertEqual(store.focusState.selectedObject, nil)
    }

    /* Snapshot tests don't pass on CI (maybe due to diff. macOS versions)

     func testSnapshotRendering() throws {
         let store = Store(engine: nil)
         let content = ContentView().environmentObject(store)
         let viewController = NSHostingController(rootView: content)
         viewController.view.frame = .init(origin: .zero, size: .init(width: 1000, height: 900))
         assertSnapshot(matching: viewController, as: .image(precision: 0.9))
     }
     */
}

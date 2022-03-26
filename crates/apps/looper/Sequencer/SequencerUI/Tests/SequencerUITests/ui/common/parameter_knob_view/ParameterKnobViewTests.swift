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

final class ParameterKnobViewTests: XCTestCase {
    func testCreateParameterKnobView() {
        let exampleParameter = FloatParameter(
            id: 0,
            globalId: .metronomeVolume,
            label: "Volume"
        )
        let parameterKnobView = ParameterKnobView(
            parameter: exampleParameter
        )
        .environmentObject(Store(engine: nil))
        .padding(20)
        .foregroundColor(SequencerColors.white)
        .background(SequencerColors.black)

        let viewController = NSHostingController(rootView: parameterKnobView)
        viewController.view.frame = CGRect(origin: .zero, size: .init(width: 100, height: 130))
        assertSnapshot(matching: viewController, as: .image)
    }
}

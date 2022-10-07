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

import AppKit
@testable import SequencerUI
import SnapshotTesting
import SwiftUI
import XCTest

// Unit-tests are using `0` stroke width and `1.0` radius because this makes
// positions easer to understand.
class MacKnobViewTests: XCTestCase {
    func testBuildValueTrackAtHalfThrough() {
        let arc = buildValueTrack(
            radius: 1.0,
            strokeWidth: 0,
            value: 0.5
        )
        XCTAssertEqual(arc.center.x, 1.0)
        XCTAssertEqual(arc.center.y, 1.0)
        XCTAssertEqual(arc.radius, 1.0)

        XCTAssertEqual(arc.startAngle, -Angle(degrees: 135).radians)
        XCTAssertEqual(arc.endAngle, -Angle(degrees: 270).radians)
    }

    func testBuildValueTrackFullyFilled() {
        let arc = buildValueTrack(
            radius: 1.0,
            strokeWidth: 0,
            value: 1.0
        )
        XCTAssertEqual(arc.center.x, 1.0)
        XCTAssertEqual(arc.center.y, 1.0)
        XCTAssertEqual(arc.radius, 1.0)

        XCTAssertEqual(arc.startAngle, -Angle(degrees: 135).radians)
        XCTAssertEqual(arc.endAngle, -Angle(degrees: 405).radians)
    }

    func testBuildCenterValueTrackCenter1() {
        let arc = buildCenterValueTrack(radius: 1.0, strokeWidth: 0.0, value: 1.0)
        XCTAssertEqual(arc.startAngle, -Angle(degrees: 270).radians)
        XCTAssertEqual(arc.endAngle, -Angle(degrees: 405).radians)
    }

    func testBuildCenterValueTrackCenterMinus1() {
        let arc = buildCenterValueTrack(radius: 1.0, strokeWidth: 0.0, value: -1.0)
        XCTAssertEqual(arc.startAngle, -Angle(degrees: 135).radians)
        XCTAssertEqual(arc.endAngle, -Angle(degrees: 270).radians)
    }

    func testGetSweepAngleNormal() {
        let sweepAngle = getSweepAngle(style: .normal)
        XCTAssertEqual(sweepAngle, Angle(degrees: 270).radians)
    }

    func testGetSweepAngleCircle() {
        let sweepAngle = getSweepAngle(style: .center)
        XCTAssertEqual(sweepAngle, Angle(degrees: 135).radians)
    }

    func testBuildPointerPath() {
        let path = buildPointerPath(
            radius: 1.0,
            style: .normal,
            strokeWidth: 0.0,
            value: 0.5
        )
        // start at (radius, radius)
        XCTAssertEqual(path.start, CGPoint(x: 1.0, y: 1.0))
        // line to (radius + radius * cos(1.0), ...)
        // this is the easy case, since we are half way through x is radius
        // and y is radius + radius; we are in the middle of the arc
        //
        // since this will lose some precision while calculating, we just check
        // the numbers are approximately equal to these two intuitive values
        XCTAssert(abs(path.end.x - 1.0) < 0.000001)
        XCTAssert(abs(path.end.y - 2.0) < 0.000001)
    }

    func testRenderKnob() {
        let knob = MacKnobView(
            value: 0.3,
            label: "Test Knob",
            formattedValue: "30%",
            style: .normal
        )

        let viewController = NSHostingController(
            rootView: knob
                .colorScheme(.dark)
                .background(SequencerColors.black)
        )
        viewController.view.frame = .init(
            origin: .zero,
            size: .init(width: 70, height: 100)
        )

        assertSnapshot(matching: viewController, as: .image(precision: 0.9))
    }

    func testRenderCenterKnob() {
        let knob = MacKnobView(
            value: 0.1,
            label: "Test Knob",
            formattedValue: "+1",
            style: .center
        )

        let viewController = NSHostingController(
            rootView: knob
                .colorScheme(.dark)
                .background(SequencerColors.black)
        )
        viewController.view.frame = .init(
            origin: .zero,
            size: .init(width: 70, height: 100)
        )

        assertSnapshot(matching: viewController, as: .image(precision: 0.9))
    }
}

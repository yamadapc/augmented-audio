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
//
//  File.swift
//
//
//  Created by Pedro Tacla Yamada on 25/3/2022.
//

@testable import SequencerUI
import XCTest

final class MIDIMappingStateStats: XCTestCase {
    func testAddMidiMessagePushesIntoState() {
        let state = MIDIMappingState()

        state.addMidiMessage(
            message: MIDIMessage(
                controllerNumber: MIDIControllerNumber(20),
                value: 80
            )
        )

        XCTAssertEqual(
            state.getLastMidiMessages().count,
            1
        )
        XCTAssertEqual(
            state.getLastMidiMessages().last!.1.controllerNumber.raw,
            20
        )
    }

    func testMessagesWithSameCCAndValueAreIgnored() {
        let state = MIDIMappingState()

        state.addMidiMessage(
            message: MIDIMessage(controllerNumber: 20, value: 80)
        )
        state.addMidiMessage(
            message: MIDIMessage(controllerNumber: 30, value: 0)
        )
        state.addMidiMessage(
            message: MIDIMessage(controllerNumber: 20, value: 80)
        )
        state.addMidiMessage(
            message: MIDIMessage(controllerNumber: 20, value: 81)
        )

        XCTAssertEqual(
            state.getLastMidiMessages().count,
            3
        )
        XCTAssertEqual(
            state.getLastMidiMessages()[0].1.controllerNumber.raw,
            20
        )
        XCTAssertEqual(
            state.getLastMidiMessages()[1].1.controllerNumber.raw,
            30
        )
        XCTAssertEqual(
            state.getLastMidiMessages()[2].1.controllerNumber.raw,
            20
        )
    }
}

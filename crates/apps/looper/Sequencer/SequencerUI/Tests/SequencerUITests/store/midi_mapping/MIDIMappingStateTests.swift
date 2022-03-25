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
            message: MIDIMessage(controllerNumber: 20, value: 80)
        )

        XCTAssertEqual(
            state.lastMidiMessages.count,
            1
        )
        XCTAssertEqual(
            state.lastMidiMessages.last!.1.controllerNumber,
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
            state.lastMidiMessages.count,
            3
        )
        XCTAssertEqual(
            state.lastMidiMessages[0].1.controllerNumber,
            20
        )
        XCTAssertEqual(
            state.lastMidiMessages[1].1.controllerNumber,
            30
        )
        XCTAssertEqual(
            state.lastMidiMessages[2].1.controllerNumber,
            20
        )
    }
}

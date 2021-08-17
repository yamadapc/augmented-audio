//
//  AudioUnitSpikeTests.swift
//  AudioUnitSpikeTests
//
//  Created by Pedro Tacla Yamada on 18/8/21.
//

import XCTest
@testable import AudioUnitSpike

class AudioUnitSpikeTests: XCTestCase {

    override func setUpWithError() throws {
        // Put setup code here. This method is called before the invocation of each test method in the class.
    }

    override func tearDownWithError() throws {
        // Put teardown code here. This method is called after the invocation of each test method in the class.
    }

    func testExample() throws {
        let results = listAllAudioUnits()
        print(results)
    }

    func testPerformanceExample() throws {
        // This is an example of a performance test case.
        self.measure {
            // Put the code you want to measure the time of here.
        }
    }

}

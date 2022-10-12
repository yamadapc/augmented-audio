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
import XCTest

class Sequencer_MacUITests: XCTestCase {
    override func setUpWithError() throws {
        continueAfterFailure = false
    }

    override func tearDownWithError() throws {}

    func testLaunch() throws {
        let app = XCUIApplication()
        app.launch()
    }

    func testLaunchAndClickTracks() throws {
        let app = XCUIApplication()
        app.launch()
        measure(metrics: [XCTCPUMetric()]) {
            app.buttons["trackButton_0"].click()
            app.buttons["trackButton_1"].click()
            app.buttons["trackButton_3"].click()
            app.buttons["trackButton_1"].click()
        }
    }

    func testLaunchAndStartRecordingThenStop() throws {
        let app = XCUIApplication()
        app.launch()
        app.buttons["recordButton"].click()
        sleep(3)
        app.buttons["recordButton"].click()
        sleep(3)
    }

    func testLaunchPerformance() throws {
        if #available(macOS 10.15, iOS 13.0, tvOS 13.0, watchOS 7.0, *) {
            // This measures how long it takes to launch your application.
            measure(metrics: [XCTApplicationLaunchMetric()]) {
                XCUIApplication().launch()
            }
        }
    }
}

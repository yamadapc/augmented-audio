//
//  MetronomeUITests.swift
//  MetronomeUITests
//
//  Created by Pedro Tacla Yamada on 28/1/2023.
//

import XCTest

final class MetronomeUITests: XCTestCase {
  override func setUpWithError() throws {
    continueAfterFailure = false
  }

  override func tearDownWithError() throws {
  }

  func testLaunch() throws {
    let app = XCUIApplication()
    setupSnapshot(app)
    app.launch()
    snapshot("0Launch")
  }
}

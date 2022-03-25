//
//  SequencerEngineTests.swift
//  SequencerEngineTests
//
//  Created by Pedro Tacla Yamada on 25/3/2022.
//

import XCTest
@testable import SequencerEngine

class SequencerEngineTests: XCTestCase {
    func testClickClear() throws {
        let engine = EngineImpl()
        engine.onClickClear(track: 1)
    }

  func testClickRecord() throws {
      let engine = EngineImpl()
      engine.onClickRecord(track: 1)
      sleep(1)
      engine.onClickRecord(track: 1)
  }
}

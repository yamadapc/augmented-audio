@testable import SequencerUI
import ViewInspector
import XCTest

final class SequencerUITests: XCTestCase {
    func testCreateStore() {
      let store = Store(engine: nil)
      XCTAssertEqual(store.focusState.selectedObject, nil)
    }

      func testBasicRendering() throws {
//        let view = SequencerView().environmentObject(Store())
//        let recordText = try! view.inspect().find(text: "Record").string()
//        XCTAssertEqual(recordText, "Record")
      }
}

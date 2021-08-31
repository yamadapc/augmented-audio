import XCTest
import ViewInspector

@testable import RecordingBuddyViews

extension EngineStateView: Inspectable {}

final class EngineStateViewTests: XCTestCase {
    func test_whenModelIsStopped_offIsRendered() throws {
        let model = EngineStateViewModel(
            isRunning: false
        )
        let subject = EngineStateView(model: model)
        let toggle = try subject.inspect().find(ViewType.Toggle.self)
        let text = try toggle.labelView().text().string()
        XCTAssertEqual(text, "Off")
    }

    func test_whenModelIsRunning_onIsRendered() throws {
        let model = EngineStateViewModel(
            isRunning: true
        )
        let subject = EngineStateView(model: model)
        let toggle = try subject.inspect().find(ViewType.Toggle.self)
        let text = try toggle.labelView().text().string()
        XCTAssertEqual(text, "On")
    }
}

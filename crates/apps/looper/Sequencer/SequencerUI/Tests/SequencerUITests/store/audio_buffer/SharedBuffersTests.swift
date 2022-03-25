@testable import SequencerUI
import XCTest

final class SharedBuffersTests: XCTestCase {
    func testUnsafeBufferTrackBuffer() {
        let buffer: [Float32] = [
            0.0,
            1.0,
        ]
        let bufferPtr = buffer.withUnsafeBufferPointer { $0 }
        let trackBuffer = UnsafeBufferTrackBuffer(
            inner: bufferPtr
        )

        XCTAssertEqual(trackBuffer.count, 2)
        XCTAssertEqual(trackBuffer[0], 0.0)
        XCTAssertEqual(trackBuffer[1], 1.0)
    }
}

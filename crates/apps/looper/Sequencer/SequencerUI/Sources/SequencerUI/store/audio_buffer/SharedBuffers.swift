import Foundation

/**
 * A shared abstract audio-buffer. This will be a shared mutable memory region, size will not change,
 * but contents may change at any point.
 */
public protocol TrackBuffer {
    var id: Int { get }
    var count: Int { get }
    subscript(_: Int) -> Float { get }

    func equals(other: TrackBuffer) -> Bool
}

/**
 * A shared abstract audio-cursors buffer.
 */
public protocol SliceBuffer {
    var id: Int { get }
    var count: Int { get }
    subscript(_: Int) -> UInt { get }

    func equals(other: SliceBuffer) -> Bool
}

/**
 * Implementation of track buffers based on `float*`.
 */
struct UnsafeBufferTrackBuffer {
    let inner: UnsafeBufferPointer<Float32>
}

extension UnsafeBufferTrackBuffer: TrackBuffer {
    var id: Int { inner.baseAddress?.hashValue ?? 0 }
    var count: Int { inner.count }
    subscript(index: Int) -> Float {
        inner[index]
    }

    func equals(other: TrackBuffer) -> Bool {
        if let otherBuffer = other as? UnsafeBufferTrackBuffer {
            return inner.baseAddress == otherBuffer.inner.baseAddress
        } else {
            return false
        }
    }
}

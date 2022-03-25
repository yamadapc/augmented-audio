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

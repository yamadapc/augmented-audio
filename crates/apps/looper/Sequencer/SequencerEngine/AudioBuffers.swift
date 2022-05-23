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

import SequencerUI
import SequencerEngine_private

class LooperBufferTrackBuffer {
    private var inner: OpaquePointer

    init(inner: OpaquePointer) {
        self.inner = inner
    }

    deinit {
        looper_buffer__free(inner)
    }
}

extension LooperBufferTrackBuffer: TrackBuffer {
    var id: Int { inner.hashValue }
    var count: Int { Int(looper_buffer__num_samples(inner)) }
    subscript(index: Int) -> Float {
        looper_buffer__get(inner, UInt(index))
    }

    func equals(other: TrackBuffer) -> Bool {
        if let otherBuffer = other as? LooperBufferTrackBuffer {
            return inner == otherBuffer.inner
        } else {
            return false
        }
    }
}

class SliceBufferImpl {
    private var inner: OpaquePointer

    init(inner: OpaquePointer) {
        self.inner = inner
    }

    deinit {
        slice_buffer__free(inner)
    }
}

extension SliceBufferImpl: SliceBuffer {
    var id: Int { inner.hashValue }
    var count: Int { Int(slice_buffer__length(inner)) }
    subscript(index: Int) -> UInt {
        slice_buffer__get(inner, UInt(index))
    }

    func equals(other: SliceBuffer) -> Bool {
        if let otherBuffer = other as? SliceBufferImpl {
            return inner == otherBuffer.inner
        } else {
            return false
        }
    }
}



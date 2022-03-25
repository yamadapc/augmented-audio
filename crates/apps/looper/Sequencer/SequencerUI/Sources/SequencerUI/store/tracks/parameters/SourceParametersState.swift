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
import Combine

public enum SourceParameterId {
    case start, end, fadeStart, fadeEnd, pitch, speed, loopEnabled, sliceId, sliceEnabled
}

public typealias SourceParameter = FloatParameter<SourceParameterId>

public class SourceParametersState: ObservableObject {
    var trackId: Int
    var start: SourceParameter
    var end: SourceParameter
    var fadeStart: SourceParameter
    var fadeEnd: SourceParameter
    var pitch: SourceParameter
    var speed: SourceParameter
    var loopEnabled: BooleanParameter
    var sliceEnabled: BooleanParameter
    var slice: IntParameter<SourceParameterId>

    public var parameters: [SourceParameter] {
        [
            start,
            end,
            fadeStart,
            fadeEnd,
            pitch,
            speed,
        ]
    }

    public var intParameters: [IntParameter<SourceParameterId>] {
        [
            slice,
        ]
    }

    public var toggles: [BooleanParameter] {
        [
            loopEnabled,
            sliceEnabled,
        ]
    }

    init(trackId: Int) {
        self.trackId = trackId
        start = SourceParameter(
            id: .start,
            globalId: .sourceParameter(trackId: trackId, parameterId: .start),
            label: "Start"
        )
        end = SourceParameter(
            id: .end,
            globalId: .sourceParameter(trackId: trackId, parameterId: .end),
            label: "End",
            initialValue: 1.0
        )
        fadeStart = SourceParameter(
            id: .fadeStart,
            globalId: .sourceParameter(trackId: trackId, parameterId: .fadeStart),
            label: "Fade start"
        )
        fadeEnd = SourceParameter(
            id: .fadeEnd,
            globalId: .sourceParameter(trackId: trackId, parameterId: .fadeEnd),
            label: "Fade end"
        )
        pitch = SourceParameter(
            id: .pitch,
            globalId: .sourceParameter(trackId: trackId, parameterId: .pitch),
            label: "Pitch",
            style: .center,
            range: (0.25, 4.0),
            initialValue: 1.0
        )
        speed = SourceParameter(
            id: .speed,
            globalId: .sourceParameter(trackId: trackId, parameterId: .speed),
            label: "Speed",
            style: .center,
            range: (-2.0, 2.0),
            initialValue: 1.0
        )

        loopEnabled = BooleanParameter(
            id: .sourceParameter(trackId: trackId, parameterId: .loopEnabled),
            label: "Loop",
            value: true
        )
        slice = IntParameter(
            id: .sourceParameter(trackId: trackId, parameterId: .sliceId),
            localId: .sliceId,
            label: "Slice",
            value: 0,
            maximum: 1
        )
        sliceEnabled = BooleanParameter(
            id: .sourceParameter(trackId: trackId, parameterId: .sliceEnabled),
            label: "Slice",
            value: false
        )
    }
}

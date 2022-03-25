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

protocol RecordingController {
    func onClickRecord()
    func onClickPlay()
    func onClickClear()
}

extension Store: RecordingController {
    func onClickRecord() {
        // try? oscClient.send(OSCMessage(
        //     with: "/looper/record"
        // ))

        engine?.onClickRecord(track: selectedTrack)
    }

    func onClickPlay() {
        // try? oscClient.send(OSCMessage(
        //     with: "/looper/play"
        // ))
        engine?.onClickPlay(track: selectedTrack)
    }

    func onClickClear() {
        // try? oscClient.send(OSCMessage(
        //     with: "/looper/clear"
        // ))
        engine?.onClickClear(track: selectedTrack)
    }
}

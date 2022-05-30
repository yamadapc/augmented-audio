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

import SequencerEngine_private
import SequencerUI

class AudioIOSettingsControllerImpl: AudioIOSettingsController {
    private let engine: OpaquePointer

    init(engine: OpaquePointer) {
        self.engine = engine
    }

    func listInputDevices() -> [AudioDevice] {
        let c_devices = audio_io_settings_controller__list_input_devices(engine)
        return toSwiftModel(c_devices)
    }

    func listOutputDevices() -> [AudioDevice] {
        let c_devices = audio_io_settings_controller__list_input_devices(engine)
        return toSwiftModel(c_devices)
    }

    func setInputDevice(_ device: AudioDevice) {
        let name = device.name
        let c_name = name.cString(using: .utf8)
        audio_io_settings_controller__set_input_device(engine, c_name)
    }

    func setOutputDevice(_ device: AudioDevice) {
        let name = device.name
        let c_name = name.cString(using: .utf8)
        audio_io_settings_controller__set_output_device(engine, c_name)
    }
}

private func toSwiftModel(_ c_devices: UnsafeMutablePointer<CAudioDeviceList>?) -> [AudioDevice] {
    var result: [AudioDevice] = []
    for i in 0 ..< audio_device_list__count(c_devices) {
        let c_device = audio_device_list__get(c_devices, i)
        let name = audio_device__name(c_device)!
        let device = AudioDevice(name: String(cString: name))
        c_string_free(name)
        result.append(device)
    }
    audio_device_list__free(c_devices)
    return result
}

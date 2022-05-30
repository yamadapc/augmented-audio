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

import SwiftUI

@available(macOS 11.0, *)
struct AudioIOPreferencesView: View {
    @EnvironmentObject var store: Store
    @State var inputDevices: [AudioDevice] = []
    @State var outputDevices: [AudioDevice] = []

    @State var inputDevice: AudioDevice?
    @State var outputDevice: AudioDevice?

    var body: some View {
        VStack(alignment: .leading) {
            Text("Audio I/O preferences")
                .bold()
                .font(.title)

            Form {
                Picker(
                    "Input device",
                    selection: $inputDevice,
                    content: {
                        ForEach(inputDevices) { device in
                            Text(device.name).tag(device as AudioDevice?)
                        }
                    }
                )
                .onChange(of: inputDevice?.id, perform: { _ in
                    guard let inputDevice = inputDevice else { return }
                    store.engine?.audioIOPreferencesController.setInputDevice(inputDevice)
                })

                Picker(
                    "Output device",
                    selection: $outputDevice,
                    content: {
                        ForEach(outputDevices) { device in
                            Text(device.name).tag(device as AudioDevice?)
                        }
                    }
                )
                .onChange(of: outputDevice?.id, perform: { _ in
                    guard let outputDevice = outputDevice else { return }
                    store.engine?.audioIOPreferencesController.setOutputDevice(outputDevice)
                })
            }
        }
        .onAppear {
            let controller = store.engine?.audioIOPreferencesController
            inputDevices = controller?.listInputDevices() ?? []
            outputDevices = controller?.listInputDevices() ?? []
        }
        .padding(PADDING)
        .frame(maxHeight: .infinity, alignment: .topLeading)
    }
}

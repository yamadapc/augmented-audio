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
public struct SettingsView: View {
    @State private var selectedTab: String? = "Settings"

    public init() {}

    public var body: some View {
        NavigationView {
            List {
                NavigationLink(
                    destination: VStack {
                        Text("Content view")
                    }
                    .frame(maxWidth: .infinity, maxHeight: .infinity),
                    tag: "Settings",
                    selection: $selectedTab,
                    label: {
                        Text("Audio settings")
                    }
                )
                NavigationLink(
                    destination: VStack {
                        Text("Content view")
                    }
                    .frame(maxWidth: .infinity, maxHeight: .infinity),
                    tag: "About",
                    selection: $selectedTab,
                    label: {
                        Text("About")
                    }
                )
            }
            .listStyle(.sidebar)
            .frame(
                maxWidth: 150,
                maxHeight: .infinity,
                alignment: .topLeading
            )
        }
        .navigationTitle("Settings")
        .frame(
            maxWidth: .infinity,
            maxHeight: .infinity
        )
    }
}

@available(macOS 11.0, *)
struct SettingsView_Previews: PreviewProvider {
    static var previews: some View {
        SettingsView()
    }
}

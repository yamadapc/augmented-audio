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
import WebView

enum PreferencesTab {
    case audioIO, about, privacy
}

@available(macOS 11.0, *)
public struct SettingsView: View {
    @State private var selectedTab: PreferencesTab? = .audioIO
    @EnvironmentObject var store: Store

    public init() {}

    public var body: some View {
        NavigationView {
            List {
                NavigationLink(
                    destination: AudioIOPreferencesView(),
                    tag: PreferencesTab.audioIO,
                    selection: $selectedTab,
                    label: {
                        Text("Audio I/O Preferences")
                    }
                )
                NavigationLink(
                    destination: VStack {
                        AboutPageView()
                    }
                    .frame(maxWidth: .infinity, maxHeight: .infinity),
                    tag: PreferencesTab.about,
                    selection: $selectedTab,
                    label: {
                        Text("About")
                    }
                )
                NavigationLink(
                    destination: PrivacyPreferencesView(),
                    tag: PreferencesTab.privacy,
                    selection: $selectedTab,
                    label: {
                        Text("Privacy")
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
        .navigationTitle("Preferences")
        .frame(
            maxWidth: .infinity,
            maxHeight: .infinity
        )
        .preferredColorScheme(.dark)
    }
}

@available(macOS 11.0, *)
struct SettingsView_Previews: PreviewProvider {
    static var previews: some View {
        SettingsView()
    }
}

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
struct PrivacyPreferencesView: View {
    @EnvironmentObject var store: Store
    @State var isAnalyticsEnabled = false

    var body: some View {
        VStack(alignment: .leading) {
            Text("Privacy preferences")
                .bold()
                .font(.title)

            Toggle(isOn: $isAnalyticsEnabled) {
                Text("Enable analytics")
                    .bold()
            }.onChange(of: isAnalyticsEnabled) { newValue in
                store.isAnalyticsEnabled = newValue
            }

            Text("If checked, Continuous Looper will collect anonymous usage and performance analytics data in order to improve its service.")
                .font(.callout)
        }
        .onAppear {
            isAnalyticsEnabled = store.isAnalyticsEnabled ?? false
        }
        .padding(PADDING)
        .frame(maxWidth: .infinity, maxHeight: .infinity, alignment: .topLeading)
    }
}

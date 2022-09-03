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

struct PlaygroundContentView: View {
    let story: PlaygroundStory

    var body: some View {
        story.view
    }
}

struct PlaygroundStory {
    let label: String
    let view: AnyView
}

func story<V: View>(_ label: String, _ builder: () -> V) -> PlaygroundStory {
    return PlaygroundStory(label: label, view: AnyView(builder()))
}

struct PlaygroundRootView: View {
    let stories: [PlaygroundStory]

    var body: some View {
        NavigationView {
            List {
                ForEach(stories, id: \.label) { item in
                    NavigationLink(
                        destination: item.view,
                        label: { Text(item.label) }
                    )
                }
            }
        }
    }
}

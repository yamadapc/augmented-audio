
import SwiftUI
@testable import SequencerUI

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
    var body: some View {
        let items: [PlaygroundStory] = [
            story("LFOVisualisationView") {
                LFOVisualisationView(lfoState: LFOState(trackId: 0, index: 0))
                    .environmentObject(Store(engine: nil))
            }
        ]

        NavigationView {
            List {
                ForEach(items, id: \.label) { item in
                    NavigationLink(
                        destination: item.view,
                        label: { Text(item.label) }
                    )
                }
            }
        }
    }
}

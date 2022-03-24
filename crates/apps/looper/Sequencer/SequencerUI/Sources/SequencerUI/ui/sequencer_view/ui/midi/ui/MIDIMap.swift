import SwiftUI

struct MIDIMapView: View {
    @ObservedObject var midi: MIDIMappingState

    var body: some View {
        VStack {
            Text("MIDI Map")
                .bold()
                .padding(PADDING)
                .frame(maxWidth: .infinity)
                .background(SequencerColors.black3)

            List(midi.mapKeys, id: \.self, rowContent: { key in
                let entry = midi.midiMap[key]
                Text("\(key.toString()) = \(String(describing: entry))")
                    .frame(maxWidth: .infinity, alignment: .leading)
            })
            .frame(maxWidth: .infinity, alignment: .leading)
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity, alignment: .top)
    }
}

import SwiftUI

struct MIDIMonitorView: View {
    @ObservedObject var midi: MIDIMappingState
    var body: some View {
        VStack {
            Text("MIDI Monitor")
                .bold()
                .padding(PADDING)
                .frame(maxWidth: .infinity)
                .background(SequencerColors.black3)

            List(
                midi.lastMidiMessages.reversed(),
                id: \.self.0,
                rowContent: { (id, message) in
                    HStack {
                        Text("CC \(message.controllerNumber) = \(message.value)")
                    }
                    .padding(PADDING)
                    .frame(maxWidth: .infinity, alignment: .leading)
                    .border(SequencerColors.black0, width: 1)
                }
            )
            .frame(maxWidth: .infinity, alignment: .leading)
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity, alignment: .top)
    }
}

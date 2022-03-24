import SwiftUI

struct MIDIMappingPanelView: View {
    var midi: MIDIMappingState

    var body: some View {
        VStack(alignment: .leading) {
            MIDIMapView(midi: midi)

            MIDIMonitorView(midi: midi)
        }
        .frame(width: 200, alignment: .topLeading)
        .frame(maxHeight: .infinity, alignment: .topLeading)
        .background(SequencerColors.black)
    }
}

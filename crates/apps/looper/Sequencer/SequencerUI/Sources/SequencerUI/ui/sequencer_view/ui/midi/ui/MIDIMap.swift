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

struct MIDIMapView: View {
    @ObservedObject var midi: MIDIMappingState

    var body: some View {
        VStack {
            Text("MIDI Map")
                .bold()
                .padding(PADDING)
                .frame(maxWidth: .infinity)
                .background(SequencerColors.black)

            List(midi.mapKeys, id: \.self, rowContent: { key in
                let entry = midi.getMapping(message: key)
                // TODO: - Add a proper view for this
                Text("\(key.toString()) = \(String(describing: entry))")
                    .frame(maxWidth: .infinity, alignment: .leading)
            })
            .frame(maxWidth: .infinity, alignment: .leading)
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity, alignment: .top)
    }
}

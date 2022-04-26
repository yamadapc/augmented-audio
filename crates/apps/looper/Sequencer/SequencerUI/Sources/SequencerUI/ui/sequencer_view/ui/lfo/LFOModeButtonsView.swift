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

enum LFOMode {
    case
        sine,
        square,
        saw
}

struct LFOModeButtonsView: View {
    @State var mode: LFOMode = .sine

    var body: some View {
        VStack(spacing: 0) {
            Picker("LFO Mode", selection: $mode) {
                Text("Sine").tag(LFOMode.sine)
                Text("Square").tag(LFOMode.square)
                Text("Saw").tag(LFOMode.saw)
            }
            .pickerStyle(.segmented)
            .padding(PADDING * 0.5)
            Rectangle()
                .fill(SequencerColors.white.opacity(0.3))
                .frame(maxWidth: .infinity, maxHeight: 1)
        }
    }
}

struct LFOModeButtonsView_Previews: PreviewProvider {
    static var previews: some View {
        LFOModeButtonsView()
    }
}

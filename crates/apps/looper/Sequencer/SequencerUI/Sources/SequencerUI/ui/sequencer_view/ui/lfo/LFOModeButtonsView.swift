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

public enum LFOMode: UInt {
    case
        sine,
        square,
        saw
}

extension LFOMode: FromRawEnum {
    public static func fromRaw(rawValue: UInt) -> LFOMode {
        return LFOMode(rawValue: rawValue)!
    }
}

struct LFOModeButtonsView: View {
    var lfoState: LFOState

    var body: some View {
        VStack(spacing: 0) {
            EnumParameterView(parameter: lfoState.modeParameter)
            Rectangle()
                .fill(SequencerColors.white.opacity(0.3))
                .frame(maxWidth: .infinity, maxHeight: 1)
        }
    }
}

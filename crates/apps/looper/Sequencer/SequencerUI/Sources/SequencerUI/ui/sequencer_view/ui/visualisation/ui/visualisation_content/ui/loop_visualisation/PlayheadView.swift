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

struct PlayheadView: View {
    // TODO: This is broken
    // @ObservedObject var position: LoopPosition
    var position: LoopPosition
    // @ObservedObject var position: LoopPosition
    var size: CGSize

    var body: some View {
        GeometryReader { geometry in
            Rectangle()
                    .fill(SequencerColors.green)
                    .frame(width: 1.0, height: size.height)
                    .position(x: 0.0, y: geometry.size.height / 2)
                    .transformEffect(
                            .init(translationX: size.width * CGFloat(position.positionPercent), y: 0.0))
        }
    }
}

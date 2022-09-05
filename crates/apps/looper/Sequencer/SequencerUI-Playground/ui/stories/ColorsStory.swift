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
import SequencerUI

func colorsStory() -> PlaygroundStory {
    return story("Colors") {
          ScrollView {
              ForEach(SequencerColors.colors.chunks(ofCount: 4), id: \.self) { colorChunk in
                  HStack {
                      ForEach(colorChunk, id: \.self) { color in
                          RoundedRectangle(cornerSize: .init(width: BORDER_RADIUS, height: BORDER_RADIUS))
                              .fill(color)
                              .frame(width: 100, height: 100, alignment: .center)
                      }
                  }
              }
              .frame(maxWidth: .infinity)
          }
          .frame(maxWidth: .infinity)
        }
}

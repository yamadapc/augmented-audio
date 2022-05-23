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
//
//  Colors.swift
//  Sequencer
//
//  Created by Pedro Tacla Yamada on 28/2/2022.
//

import Foundation
import SwiftUI

func color(_ r: Double, _ g: Double, _ b: Double) -> Color {
    return Color(red: r / 255, green: g / 255, blue: b / 255)
}

// enum SequencerColors {
//    static let transparent = Color.white.opacity(0)
//    static let black = color(20, 20, 20)
//    static let black0 = color(0, 0, 0)
//    static let black1 = color(20 * 2, 20 * 2, 23 * 2)
//    static let black3 = color(20 * 3, 20 * 3, 20 * 3)
//    static let white = color(245, 239, 237)
//    static let red = color(255, 103, 77)
//    static let green = color(77, 139, 49)
//    static let blue0 = color(8 * 0.05, 150 * 0.05, 180 * 0.05)
//    static let blue = color(8, 178, 227)
//    static let yellow = color(204, 156, 0)
//    // static let yellow = color(245, 187, 0)
// }

enum SequencerColors {
    static let transparent = Color.white.opacity(0)
    static let black0 = color(20, 20, 20)
    static let black = color(0, 0, 0)
    static let black1 = color(20 * 2, 20 * 2, 23 * 2)
    static let black3 = color(20 * 3, 20 * 3, 20 * 3)
    static let white = color(245, 239, 237)
    private static let red = color(255, 103, 77)
    static let recordColor = red
    static let selectionColorHigh = red
    static let green = color(77, 139, 49)
    static let blue0 = color(8 * 0.05, 150 * 0.05, 180 * 0.05)
    static let blue = color(8, 178, 227)
    static let yellow = color(204, 156, 0)
    static let purple1 = color(1, 23, 47)

    static let tooltipColor = color(194, 0, 251)

    static let colors = [
        green,
        blue,
        yellow,
        green,
        purple1,
        color(255, 22, 84),
        color(120, 202, 210),
        color(238, 252, 87),
        color(65, 64, 102),
        color(254, 201, 241),
        color(96, 49, 64),
        color(48, 242, 242),
        color(194, 0, 251),
    ]

    static func randomColor<H: Hashable>(_ id: H) -> Color {
        let hash = abs(id.hashValue)
        return colors[hash % colors.count]
    }
}

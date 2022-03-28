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
//  File.swift
//
//
//  Created by Pedro Tacla Yamada on 22/3/2022.
//

import SwiftUI

struct CPUMeterView: View {
    @ObservedObject var processorMetrics: ProcessorMetrics

    var body: some View {
        let stats = processorMetrics.getStats()
        Text("\(String(format: "%.0f", stats.maximumCpu * 100))%")
            .frame(width: 50, alignment: .trailing)
            .padding(PADDING / 2.0)
            .background(SequencerColors.black0)
            .cornerRadius(BORDER_RADIUS / 2)
    }
}

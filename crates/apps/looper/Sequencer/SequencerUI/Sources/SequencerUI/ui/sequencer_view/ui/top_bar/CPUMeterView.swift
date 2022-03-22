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
      Text("\(String(format: "%.0f", processorMetrics.inner.maximumCpu * 100))%")
        .frame(width: 50, alignment: .trailing)
    }
}

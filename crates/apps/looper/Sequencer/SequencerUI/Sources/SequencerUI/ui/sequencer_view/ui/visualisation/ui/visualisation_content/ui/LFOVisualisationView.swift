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

struct LFOVisualisationViewInner: View {
    @ObservedObject var lfoAmount: FloatParameter
    @ObservedObject var lfoFrequency: FloatParameter
    @State var tick: Int = 0
    @State var lastTranslation: CGSize = .zero

    var body: some View {
        GeometryReader { geometry in
            ZStack {
                Path { path in
                    buildPath(geometry, &path, tick)
                }
                .stroke(SequencerColors.blue, lineWidth: 2)
                .contentShape(Rectangle())
                .gesture(
                    DragGesture(minimumDistance: 0)
                        .onChanged { drag in
                            let currentTranslation = drag.translation

                            var amount = CGFloat(lfoAmount.value)
                            amount -= (currentTranslation.height - lastTranslation.height) / (geometry.size.height / 2)
                            amount = max(min(amount, 1), 0)
                            lfoAmount.value = Float(amount)

                            var frequency = CGFloat(lfoFrequency.value)
                            frequency -= (currentTranslation.width - lastTranslation.width) / (geometry.size.width / 2)
                            frequency = min(max(frequency, 0.01), 20)
                            lfoFrequency.value = Float(frequency)

                            self.lastTranslation = currentTranslation
                        }
                        .onEnded { _ in
                            self.lastTranslation = CGSize.zero
                        }
                )

                VStack(alignment: .trailing) {
                    Text("Amount: \(String(format: "%.0f", lfoAmount.value * 100))%")
                    Text("Frequency: \(String(format: "%.2f", lfoFrequency.value))Hz")
                }
                .padding(PADDING)
                .border(SequencerColors.blue.opacity(0.5), width: 1)
                .background(SequencerColors.black.opacity(0.7))
                .frame(maxWidth: .infinity, maxHeight: .infinity, alignment: .bottomTrailing)
                .allowsHitTesting(false)
            }
        }
        .padding(PADDING)
    }

    func buildPath(_ geometry: GeometryProxy, _ path: inout Path, _ tick: Int) {
        let height = geometry.size.height
        let maxH = height / 2
        let width = Int(geometry.size.width)
        let baseWidth = (Double(width) / 32) // 1Hz repr
        let maxWidth = baseWidth / (CGFloat(lfoFrequency.value) / 2)

        for x in 0 ... width {
            let value = sin(Double(x + tick) / maxWidth)
            let h = value * maxH * CGFloat(lfoAmount.value) + maxH

            if x == 0 {
                path.move(to: CGPoint(x: Double(x), y: h))
            }
            path.addLine(to: CGPoint(x: Double(x), y: h))
        }
    }
}

struct LFOVisualisationView: View {
    var lfoState: LFOState

    var body: some View {
        LFOVisualisationViewInner(lfoAmount: lfoState.amountParameter, lfoFrequency: lfoState.frequencyParameter)
    }
}

//
//  File.swift
//
//
//  Created by Pedro Tacla Yamada on 11/3/2022.
//

import SwiftUI

protocol LFOVisualisationViewModel: ObservableObject {
    var frequency: Double { get set }
    var amount: Double { get set }
}

struct LFOVisualisationView<T: LFOVisualisationViewModel>: View {
    @ObservedObject var model: T

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

                            model.amount -= (currentTranslation.height - lastTranslation.height) / (geometry.size.height / 2)
                            model.amount = max(min(model.amount, 1), 0)

                            model.frequency -= (currentTranslation.width - lastTranslation.width) / (geometry.size.width / 2)
                            model.frequency = min(max(model.frequency, 0.01), 20)

                            self.lastTranslation = currentTranslation
                        }
                        .onEnded { _ in
                            self.lastTranslation = CGSize.zero
                        }
                )

                VStack(alignment: .trailing) {
                    Text("Amount: \(String(format: "%.0f", model.amount * 100))%")
                    Text("Frequency: \(String(format: "%.2f", model.frequency))Hz")
                }
                .padding()
                .border(SequencerColors.blue.opacity(0.5), width: 1)
                .background(SequencerColors.black.opacity(0.7))
                .frame(maxWidth: .infinity, maxHeight: .infinity, alignment: .bottomTrailing)
            }
        }
        .padding()
    }

    func buildPath(_ geometry: GeometryProxy, _ path: inout Path, _ tick: Int) {
        let height = geometry.size.height
        let maxH = height / 2
        let width = Int(geometry.size.width)
        let baseWidth = (Double(width) / 32) // 1Hz repr
        let maxWidth = baseWidth / (model.frequency / 2)

        for x in 0 ... width {
            let value = sin(Double(x + tick) / maxWidth)
            let h = value * maxH * model.amount + maxH

            if x == 0 {
                path.move(to: CGPoint(x: Double(x), y: h))
            }
            path.addLine(to: CGPoint(x: Double(x), y: h))
        }
    }
}

//
//  File.swift
//
//
//  Created by Pedro Tacla Yamada on 17/3/2022.
//

import SwiftUI

struct ParameterLockAnimationView: View {
    @ObservedObject var focusState: FocusState
    var parameterId: ObjectId
    @State var isAnimating = false

    var body: some View {
        if focusState.draggingStep != nil && focusState.mouseOverObject == parameterId {
            ZStack {
                Circle()
                    .stroke(SequencerColors.green, lineWidth: 2)
                    .frame(width: 30 * 2, height: 30 * 2)
                    .opacity(self.isAnimating ? 0.0 : 1.0)
                    .scaleEffect(self.isAnimating ? 1.5 : 0.0, anchor: .center)
                    .animation(
                        .linear(duration: 0.75).repeatForever(autoreverses: false),
                        value: self.isAnimating
                    )
                Circle()
                    .stroke(SequencerColors.green, lineWidth: 2.0)
                    .frame(width: 20 * 2, height: 20 * 2)
                    .opacity(self.isAnimating ? 0.0 : 1.0)
                    .scaleEffect(self.isAnimating ? 1.5 : 0.0, anchor: .center)
                    .animation(
                        .linear(duration: 0.75).repeatForever(autoreverses: false),
                        value: self.isAnimating
                    )
                Circle()
                    .stroke(SequencerColors.green, lineWidth: 2)
                    .frame(width: 10 * 2, height: 10 * 2)
                    .opacity(self.isAnimating ? 0.0 : 1.0)
                    .scaleEffect(self.isAnimating ? 1.5 : 0.0, anchor: .center)
                    .animation(
                        .linear(duration: 0.75).repeatForever(autoreverses: false),
                        value: self.isAnimating
                    )
            }
            .animation(
                .linear(duration: 0.75).repeatForever(autoreverses: false),
                value: self.isAnimating
            )
            .onAppear {
                self.isAnimating = true
            }
            .onDisappear {
                self.isAnimating = false
            }
        }
    }
}

struct ParameterKnobView<ParameterId>: View {
    @ObservedObject var parameter: FloatParameter<ParameterId>
    @EnvironmentObject var store: Store

    var body: some View {
        KnobView(
            label: parameter.label,
            onChanged: { newValue in
                let mappedValue = parameterFromKnobValue(knobValue: newValue)
                parameter.setValue(mappedValue)
            },
            onEnded: {
                store.endParameterLock(parameter)
            },
            formatValue: { _ in
                String(format: "%.2f", parameter.parameterLockProgress?.newValue ?? parameter.value)
            },
            value: Double(
                parameterToKnobValue()
            )
        )
        .style(parameter.style)
        .gesture(
            TapGesture(count: 2).onEnded {
                parameter.setValue(parameter.defaultValue)
            }
        )
        .bindToParameter(store: store, parameter: parameter)
        .overlay(
            ParameterLockAnimationView(
                focusState: store.focusState,
                parameterId: parameter.globalId
            ),
            alignment: .center
        )
    }

    func parameterFromKnobValue(knobValue: Double) -> Float {
        let (min, max) = parameter.range
        // number between 0-1
        let result = Float(
            parameter.style == .center
                ? (knobValue + 1.0) / 2.0
                : knobValue
        ) * (max - min) + min

        return result
    }

    func parameterToKnobValue() -> Float {
        let value = parameter.parameterLockProgress?.newValue ?? parameter.value
        let (min, max) = parameter.range
        // number between 0-1
        let knobValue = (value - min) / (max - min)
        if parameter.style == .center {
            return knobValue * 2.0 + -1.0
        }

        return knobValue
    }
}

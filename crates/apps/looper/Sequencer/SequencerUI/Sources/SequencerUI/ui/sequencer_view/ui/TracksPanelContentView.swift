//
//  SwiftUIView.swift
//
//
//  Created by Pedro Tacla Yamada on 11/3/2022.
//

import OSCKit
import SwiftUI

struct LFOKnobsView: View {
    @EnvironmentObject var store: Store
    @ObservedObject var lfoState: LFOState

    var body: some View {
        HStack {
            KnobView(
                label: "LFO amount",
                onChanged: { value in
                    lfoState.amount = value
                },
                formatValue: { value in
                    "\(String(format: "%.0f", value * 100))%"
                },
                value: lfoState.amount
            )
            .bindToParameter(store: store, parameter: lfoState.amountParameter)

            KnobView(
                label: "LFO frequency",
                onChanged: { value in
                    lfoState.frequency = value * (20 - 0.01) + 0.01
                },
                formatValue: { value in
                    let frequency = value * (20 - 0.01) + 0.01
                    return "\(String(format: "%.2f", frequency))Hz"
                },
                value: (lfoState.frequency - 0.01) / (20 - 0.01)
            )
            .bindToParameter(store: store, parameter: lfoState.frequencyParameter)
        }
    }
}

struct MixKnobView: View {
    var trackId: Int

    @EnvironmentObject var store: Store
    @ObservedObject var trackState: TrackState

    @State var value: Float = 0

    var body: some View {
        ZStack {
            Text("\(trackState.volumeParameter.label)")
                .foregroundColor(SequencerColors.white.opacity(0.4))
                .rotationEffect(Angle(degrees: -90))
                .transformEffect(.init(translationX: 15, y: 0))

            KnobSliderView(
                value: $trackState.volumeParameter.value,
                defaultValue: trackState.volumeParameter.defaultValue,
                style: .vertical,
                tickColor: SequencerColors.black3,
                railColor: SequencerColors.black3,
                handleColor: SequencerColors.black0
            )
            .bindToParameterId(store: store, parameterId: trackState.volumeParameter.globalId)
            // ParameterKnobView(parameter: trackState.volumeParameter)
        }
    }
}

struct MixPanelContentView: View {
    @EnvironmentObject var store: Store

    var body: some View {
        HStack {
            ForEach(1 ..< 9) { i in
                HStack(spacing: 0) {
                    Rectangle()
                        .fill(SequencerColors.black)
                        .frame(width: 1)
                        .frame(maxHeight: .infinity)

                    MixKnobView(trackId: i, trackState: store.trackStates[i - 1])
                        .frame(width: 78)

                    Rectangle()
                        .fill(SequencerColors.black)
                        .frame(width: 1)
                        .frame(maxHeight: .infinity)
                }
            }

            ParameterKnobView(parameter: store.metronomeVolume)
        }
        .padding(EdgeInsets(top: 0, leading: PADDING, bottom: 0, trailing: PADDING))
    }
}

struct SourcePanelContentView: View {
    @ObservedObject var sourceParameters: SourceParametersState

    var body: some View {
        HStack(alignment: .center, spacing: 30) {
            ForEach(sourceParameters.parameters) { parameter in
                ParameterKnobView(
                    parameter: parameter
                )
            }

            ForEach(sourceParameters.intParameters) { parameter in
                ParameterKnobView(
                    parameter: parameter
                )
            }
        }
    }
}

struct EnvelopePanelContentView: View {
    @ObservedObject var envelope: EnvelopeState

    var body: some View {
        HStack(alignment: .center, spacing: 30) {
            ForEach(envelope.parameters) { parameter in
                ParameterKnobView(parameter: parameter)
            }
        }
    }
}

struct EffectsPanelContentView: View {
    var body: some View {
        HStack {
            Text("Hello world")
        }
    }
}

struct TracksPanelContentView: View {
    @EnvironmentObject var store: Store

    var body: some View {
        HStack {
            let tracksPanelContentView = HStack(alignment: .top, spacing: 30) {
                switch store.selectedTab {
                case .mix:
                    MixPanelContentView()
                case .source, .slice:
                    SourcePanelContentView(
                        sourceParameters: store.currentTrackState().sourceParameters
                    )

                case .envelope:
                    EnvelopePanelContentView(
                        envelope: store.currentTrackState().envelope
                    )
                case .fx:
                    EffectsPanelContentView()
                case .lfos:
                    HStack(spacing: PADDING) {
                        LFOKnobsView(lfoState: store.currentTrackState().lfo1)
                        LFOKnobsView(lfoState: store.currentTrackState().lfo2)
                    }
                }
            }
            .padding(PADDING * 2)
            .frame(maxWidth: .infinity, maxHeight: .infinity)

            tracksPanelContentView
        }.frame(maxHeight: .infinity)
            .foregroundColor(SequencerColors.white)
            .background(SequencerColors.black)
    }
}

struct TracksPanelContentView_Previews: PreviewProvider {
    static var previews: some View {
        TracksPanelContentView().environmentObject(Store(engine: nil))
    }
}

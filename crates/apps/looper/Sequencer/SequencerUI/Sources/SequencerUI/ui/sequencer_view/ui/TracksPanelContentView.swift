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
                radius: 20,
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
                radius: 20,
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
            .bindToParameter(store: store, parameter: lfoState.amountParameter)
        }
        .padding(PADDING)
    }
}

struct MixKnobView: View {
    @EnvironmentObject var store: Store
    var trackId: Int
    @ObservedObject var trackState: TrackState

    var body: some View {
        ParameterKnobView(parameter: trackState.volumeParameter)
    }
}

struct MixPanelContentView: View {
    @EnvironmentObject var store: Store

    var body: some View {
        HStack(alignment: .center, spacing: 30) {
            ForEach(1 ..< 9) { i in
                MixKnobView(trackId: i, trackState: store.trackStates[i - 1])
            }

            ParameterKnobView(parameter: store.metronomeVolume)
        }
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
                }
            }
            .padding(PADDING * 2)
            .frame(maxWidth: .infinity, maxHeight: .infinity)

            HStack {
                tracksPanelContentView

                HStack(spacing: 0) {
                    VStack {
                        LFOVisualisationView(model: store.currentTrackState().lfo1)
                            .background(SequencerColors.black0)
                        LFOKnobsView(lfoState: store.currentTrackState().lfo1)
                    }
                    .border(SequencerColors.black1, width: 1)

                    VStack {
                        LFOVisualisationView(model: store.currentTrackState().lfo2)
                            .background(SequencerColors.black0)
                        LFOKnobsView(lfoState: store.currentTrackState().lfo2)
                    }
                    .border(SequencerColors.black1, width: 1)
                }
            }
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

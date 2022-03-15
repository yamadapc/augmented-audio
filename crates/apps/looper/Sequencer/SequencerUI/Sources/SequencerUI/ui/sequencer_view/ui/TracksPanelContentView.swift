//
//  SwiftUIView.swift
//
//
//  Created by Pedro Tacla Yamada on 11/3/2022.
//

import OSCKit
import SwiftUI

struct LFOKnobsView: View {
    @ObservedObject var lfoState: LFOState

    var body: some View {
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
    }
}

struct MixKnobView: View {
    @EnvironmentObject var store: Store
    var trackId: Int
    @ObservedObject var trackState: TrackState

    var body: some View {
        KnobView(
            label: "Volume \(trackId)",
            onChanged: { volume in
                store.setVolume(track: trackId, volume: Float(volume))
            },
            value: Double(trackState.volume)
        )
    }
}

struct MixPanelContentView: View {
    @EnvironmentObject var store: Store

    var body: some View {
        HStack(alignment: .center, spacing: 30) {
            ForEach(1 ..< 9) { i in
                MixKnobView(trackId: i, trackState: store.trackStates[i - 1])
            }
        }
    }
}

struct TracksPanelContentView: View {
    @EnvironmentObject var store: Store

    var body: some View {
        HStack {
            let tracksPanelContentView = HStack(alignment: .top, spacing: 30) {
                switch store.selectedTab {
                case .lfos:
                    HStack(alignment: .center, spacing: 30) {
                        LFOKnobsView(lfoState: store.currentTrackState().lfo1)
                        LFOKnobsView(lfoState: store.currentTrackState().lfo2)
                    }

                case .mix:
                    MixPanelContentView()
                case .source:
                    HStack(alignment: .center, spacing: 30) {
                        KnobView(label: "Start", value: 0)
                        KnobView(label: "End")
                        KnobView(label: "Fade start", value: 0)
                        KnobView(label: "Fade end", value: 0)
                        KnobView(label: "Pitch", value: 0).style(.center)
                        KnobView(label: "Speed", value: 0).style(.center)
                    }

                default:
                    HStack(alignment: .center, spacing: 30) {
                        KnobView(label: "Normal")
                        KnobView(label: "Center", value: 0.1).style(.center)
                        KnobView(label: "Other")
                        KnobView().style(.center)
                        KnobView()
                        KnobView().style(.center)
                        KnobView()

                        KnobView(
                            onChanged: { value in
                                store.setParameter(name: "volume", value: Float(value))
                            }
                        )
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

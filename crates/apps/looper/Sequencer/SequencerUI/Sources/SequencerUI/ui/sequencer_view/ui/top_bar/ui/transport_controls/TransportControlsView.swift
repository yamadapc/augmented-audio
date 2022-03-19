//
//  File.swift
//
//
//  Created by Pedro Tacla Yamada on 12/3/2022.
//

import SwiftUI

extension Text {
    func monospacedDigitCompat() -> Text {
        if #available(macOS 12.0, *) {
            return self.monospacedDigit()
        } else {
            return self
        }
    }
}

struct TransportTempoView: View {
    @EnvironmentObject var store: Store
    @ObservedObject var timeInfo: TimeInfo

    @State var previousX = 0.0

    var body: some View {
        let content = self.getTextContent()

        HStack {
            Text(content)
                .monospacedDigitCompat()
                .gesture(DragGesture().onChanged { data in
                    var tempo = timeInfo.tempo ?? 120.0
                    let deltaX = data.translation.width - previousX
                    self.previousX = data.translation.width
                    tempo += Double(deltaX) / 100.0
                    store.setTempo(tempo: Float(tempo))
                }.onEnded { _ in
                    self.previousX = 0
                })
        }
        .padding(PADDING * 0.5)
        .background(SequencerColors.black3)
    }

    func getTextContent() -> String {
        if let tempo = timeInfo.tempo {
            return "\(String(format: "%.1f", tempo))bpm"
        } else {
            return "Free tempo"
        }
    }
}

struct TransportBeatsView: View {
    @ObservedObject var timeInfo: TimeInfo

    var body: some View {
        if let beats = timeInfo.positionBeats {
            Text("\(String(format: "%.1f", 1.0 + Float(Int(beats * 10) % 40) / 10.0))")
                .monospacedDigitCompat()
        } else {
            Text("0.0")
        }
    }
}

struct TransportControlsView: View {
    @EnvironmentObject var store: Store

    var body: some View {
        HStack(alignment: .center) {
            TransportBeatsView(timeInfo: store.timeInfo).frame(width: 30, alignment: .trailing)

            Rectangle().fill(SequencerColors.black3).frame(width: 1.0, height: 10.0)

            Button(action: {
                store.onClickPlayheadPlay()
            }) {
                if #available(macOS 11.0, *) {
                    Image(systemName: "play.fill")
                        .renderingMode(.template)
                        .foregroundColor(store.isPlaying ? SequencerColors.green : SequencerColors.white)
                } else {
                    Text("Play")
                }
            }.buttonStyle(.plain).frame(maxHeight: .infinity)
            .bindToParameterId(store: store, parameterId: .transportPlay)

            Button(action: {
                store.onClickPlayheadStop()
            }) {
                if #available(macOS 11.0, *) {
                    Image(systemName: "stop.fill")
                } else {
                    Text("Stop")
                }
            }.buttonStyle(.plain).frame(maxHeight: .infinity)
            .bindToParameterId(store: store, parameterId: .transportStop)
        }
        .frame(maxHeight: 30)
    }
}

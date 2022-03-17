//
//  File.swift
//
//
//  Created by Pedro Tacla Yamada on 12/3/2022.
//

import SwiftUI

struct TransportTempoView: View {
    @ObservedObject var timeInfo: TimeInfo

    var body: some View {
        if let tempo = timeInfo.tempo {
            if #available(macOS 12.0, *) {
                Text("\(String(format: "%.1f", tempo))bpm")
                    .monospacedDigit()
            } else {
                Text("\(String(format: "%.1f", tempo))bpm")
            }
        } else {
            Text("Free tempo")
        }
    }
}

struct TransportBeatsView: View {
    @ObservedObject var timeInfo: TimeInfo

    var body: some View {
        if let beats = timeInfo.positionBeats {
            if #available(macOS 12.0, *) {
                Text("\(String(format: "%.1f", 1.0 + Float(Int(beats * 10) % 40) / 10.0))")
                    .monospacedDigit()
            } else {
                Text("\(String(format: "%.1f", 1.0 + Float(Int(beats * 10) % 40) / 10.0))")
            }
        } else {
            Text("0.0")
        }
    }
}

struct TransportInfoView: View {
    @EnvironmentObject var store: Store
    var body: some View {
        HStack {
            TransportTempoView(timeInfo: store.timeInfo).frame(width: 80, alignment: .trailing)
            Rectangle().fill(SequencerColors.black3).frame(width: 1.0, height: 10.0)
            TransportBeatsView(timeInfo: store.timeInfo).frame(width: 30, alignment: .trailing)
        }.frame(width: 200.0)
    }
}

struct TransportControlsView: View {
    @EnvironmentObject var store: Store

    var body: some View {
        HStack(alignment: .center) {
            TransportInfoView()

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

            Button(action: {
                store.onClickPlayheadStop()
            }) {
                if #available(macOS 11.0, *) {
                    Image(systemName: "stop.fill")
                } else {
                    Text("Stop")
                }
            }.buttonStyle(.plain).frame(maxHeight: .infinity)
        }
        .frame(maxWidth: .infinity, maxHeight: 50)
    }
}

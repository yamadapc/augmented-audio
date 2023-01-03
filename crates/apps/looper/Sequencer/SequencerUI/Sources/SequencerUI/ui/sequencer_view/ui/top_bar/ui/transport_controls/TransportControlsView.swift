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
//  Created by Pedro Tacla Yamada on 12/3/2022.
//

import Combine
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

class TransportTempoViewModel: ObservableObject {
    var timeInfo: TimeInfo
    @Published var tempoString: String = "Free tempo"
    var cancellables: Set<AnyCancellable> = Set()

    init(timeInfo: TimeInfo) {
        self.timeInfo = timeInfo
        tempoString = getTextContent(tempo: timeInfo.tempo)
        self.timeInfo.$tempo.sink(receiveValue: { tempo in
            let tempoString = self.getTextContent(tempo: tempo)
            if tempoString != self.tempoString {
                self.tempoString = tempoString
            }
        }).store(in: &cancellables)
    }

    func getTextContent(tempo: Double?) -> String {
        if let tempo = tempo {
            return "\(String(format: "%.1f", tempo))bpm"
        } else {
            return "Free tempo"
        }
    }
}

struct TransportTempoView: View {
    @ObservedObject var model: TransportTempoViewModel

    @EnvironmentObject var store: Store
    @State var previousX = 0.0

    var body: some View {
        HStack {
            Text(model.tempoString)
                .monospacedDigitCompat()
                .gesture(
                    DragGesture().onChanged { data in
                        var tempo = store.timeInfo.tempo ?? 120.0
                        let deltaX = data.translation.width - previousX
                        self.previousX = data.translation.width
                        tempo += Double(deltaX) / 100.0
                        store.setTempo(tempo: Float(tempo))
                    }
                    .onEnded { _ in
                        self.previousX = 0
                    }
                )
        }
        .padding(PADDING * 0.5)
        .background(SequencerColors.black3)
        .cornerRadius(BORDER_RADIUS / 2)
    }
}

#if os(macOS)
    class TimeInfoTextView: NSView {
        var view = NSTextView()
        var timeInfo: TimeInfo = TimeInfo() {
            didSet {
                cancellables = Set()
                timeInfo.objectWillChange.sink(receiveValue: { _ in
                    DispatchQueue.main.async {
                        self.view.string = self.getText()
                    }
                })
                .store(in: &cancellables)
            }
        }

        var cancellables: Set<AnyCancellable> = Set()

        init(timeInfo: TimeInfo) {
            self.timeInfo = timeInfo

            super.init(frame: NSRect.zero)

            view.string = getText()
            view.isEditable = false
            view.isSelectable = false
            view.isRichText = false
            view.drawsBackground = false

            self.addSubview(view)

            self.wantsLayer = true
            self.layer?.backgroundColor = NSColor.red.cgColor
            view.frame = self.frame

            self.addConstraint(view.topAnchor.constraint(equalTo: self.topAnchor))
            self.translatesAutoresizingMaskIntoConstraints = true
            self.autoresizingMask = []
            view.translatesAutoresizingMaskIntoConstraints = true
            view.autoresizingMask = [.width]
        }

        required init?(coder: NSCoder) {
            self.timeInfo = TimeInfo()
            super.init(coder: coder)
        }

        func getText() -> String {
            if let beats = timeInfo.positionBeats {
                return "\(String(format: "%.1f", 1.0 + Float(Int(beats * 10) % 40) / 10.0))"
            } else {
                return "0.0"
            }
        }
    }

    struct NativeTransportBeats: NSViewRepresentable {
        var timeInfo: TimeInfo
        typealias NSViewType = TimeInfoTextView

        func makeNSView(context _: Context) -> TimeInfoTextView {
            let view = TimeInfoTextView(timeInfo: timeInfo)
            return view
        }

        func updateNSView(_ view: TimeInfoTextView, context _: Context) {
            view.timeInfo = timeInfo
        }
    }
#else
    struct NativeTransportBeats: UIViewRepresentable {
        typealias UIViewType = UITextView

        var timeInfo: TimeInfo
        var cancellables: Set<AnyCancellable> = Set()

        init(timeInfo: TimeInfo) {
            self.timeInfo = timeInfo
        }

        func makeUIView(context _: Context) -> UIViewType {
            let view = UITextView()
            view.text = getText()
            view.isEditable = false
            view.isSelectable = false
            timeInfo.objectWillChange.sink(receiveValue: { _ in
                DispatchQueue.main.async {
                    view.text = self.getText()
                }
            })
            .store(in: &cancellables)
            return view
        }

        func updateUIView(_: UIViewType, context _: Context) {}

        func getText() -> String {
            if let beats = timeInfo.positionBeats {
                return "\(String(format: "%.1f", 1.0 + Float(Int(beats * 10) % 40) / 10.0))"
            } else {
                return "0.0"
            }
        }
    }
#endif

struct TransportBeatsView: View {
    var timeInfo: TimeInfo

    var body: some View {
        NativeTransportBeats(timeInfo: timeInfo)
            .padding(EdgeInsets(top: PADDING, leading: 0, bottom: 0, trailing: 0))
    }

    func getText() -> String {
        if let beats = timeInfo.positionBeats {
            return "\(String(format: "%.1f", 1.0 + Float(Int(beats * 10) % 40) / 10.0))"
        } else {
            return "0.0"
        }
    }
}

struct TransportControlsView: View {
    @EnvironmentObject var store: Store

    var body: some View {
        HStack(alignment: .center) {
            TransportBeatsView(timeInfo: store.timeInfo)
            .frame(width: 30, alignment: .trailing)
            .offset(y: -5)

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
            }
            .buttonStyle(.plain)
            .frame(maxHeight: .infinity)
            .bindToParameterId(
                store: store,
                parameterId: .transportPlay,
                showSelectionOverlay: false
            )

            Button(action: {
                store.onClickPlayheadStop()
            }) {
                if #available(macOS 11.0, *) {
                    Image(systemName: "stop.fill")
                } else {
                    Text("Stop")
                }
            }
            .buttonStyle(.plain)
            .frame(maxHeight: .infinity)
            .bindToParameterId(
                store: store,
                parameterId: .transportStop,
                showSelectionOverlay: false
            )
        }
        .frame(maxHeight: 30)
    }
}

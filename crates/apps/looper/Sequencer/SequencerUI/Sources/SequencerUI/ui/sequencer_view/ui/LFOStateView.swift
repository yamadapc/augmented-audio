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

struct LFOStateSingleView: View {
    var lfoState: LFOState

    @State var dragPosition: (CGPoint, CGPoint)?
    @EnvironmentObject var store: Store

    var body: some View {
        ZStack {
            VStack(spacing: 0) {
                LFOVisualisationView(lfoState: lfoState)
                LFOPanelContentView(lfoState: lfoState)
            }

            if let (start, end) = dragPosition {
                ZStack(alignment: .top) {
                    Rectangle()
                        .fill(SequencerColors.blue.opacity(0.4))
                        .border(SequencerColors.white, width: 1)

                    Text("Map \(lfoState.label)")
                        .bold()
                        .padding(PADDING)
                        .background(Rectangle().fill(SequencerColors.black0.opacity(0.8)))
                        .cornerRadius(BORDER_RADIUS)
                        .offset(y: PADDING)
                }
                .opacity(dragPosition != nil ? 1.0 : 0.0)
                .frame(maxWidth: .infinity, maxHeight: .infinity)
                .padding(2)

                Path { path in
                    path.move(to: start)
                    path.addLine(to: end)
                }.stroke(SequencerColors.green, lineWidth: 3)
            }
        }
        .background(SequencerColors.black)
        .apply(buildDragGesture)
    }

    func buildDragGesture<C: View>(_ view: C) -> some View {
      #if os(macOS)
        view.highPriorityGesture(
                DragGesture()
                        .modifiers(.command)
                        .onChanged { drag in
                            store.startDrag(source: .lfoId(lfoState.id), dragMode: .lock)
                            DispatchQueue.main.async {
                                withAnimation(.spring()) {
                                    self.dragPosition = (drag.startLocation, drag.location)
                                }
                            }
                        }
                        .onEnded { _ in
                            DispatchQueue.main.async {
                                self.dragPosition = nil
                                store.endGlobalDrag()
                            }
                        }
        )
        #else
        return view
        #endif
    }
}

struct LFOStateView: View {
    @EnvironmentObject var store: Store

    var body: some View {
        HStack(spacing: 0) {
            LFOStateSingleView(lfoState: store.currentTrackState().lfo1)
                .overlay(
                    Rectangle()
                        .frame(width: 1, height: nil, alignment: .trailing)
                        .foregroundColor(SequencerColors.black3),
                    alignment: .trailing
                )

            LFOStateSingleView(lfoState: store.currentTrackState().lfo2)
        }
    }
}

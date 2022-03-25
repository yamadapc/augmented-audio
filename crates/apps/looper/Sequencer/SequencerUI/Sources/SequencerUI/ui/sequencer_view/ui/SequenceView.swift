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
//  SwiftUIView.swift
//
//
//  Created by Pedro Tacla Yamada on 11/3/2022.
//

import Combine
import SwiftUI

struct SequenceModel {
    var activeSteps: Set<Int>
}

struct TrackButtonView: View {
    var isBeat: Bool
    var isActive: Bool
    var isPlaying: Bool
    var hasLocks: Bool

    var onClick: () -> Void

    var body: some View {
        Button(
            action: onClick,
            label: {
                Text("")
                    .frame(maxWidth: .infinity, maxHeight: 40, alignment: .center)
                    .contentShape(Rectangle())
                    .foregroundColor(SequencerColors.white)
                    .overlay(
                        RoundedRectangle(cornerRadius: BORDER_RADIUS)
                            .stroke(SequencerColors.black3, lineWidth: 1.0)
                    )
                    .background(
                        hasLocks
                            ? SequencerColors.green
                            : isActive
                            ? SequencerColors.blue
                            : isBeat ? SequencerColors.black0 : SequencerColors.black
                    )
                    .cornerRadius(BORDER_RADIUS)
            }
        )
        .buttonStyle(.plain)
        .opacity(
            isPlaying
                ? 0.3
                : 1.0
        )
    }
}

extension TrackButtonView: Equatable {
    static func == (lhs: TrackButtonView, rhs: TrackButtonView) -> Bool {
        lhs.isPlaying == rhs.isPlaying && lhs.isBeat == rhs.isBeat && lhs.isActive == rhs.isActive && lhs.hasLocks == rhs.hasLocks
    }
}

class StepButtonViewModel: ObservableObject {
    var store: Store
    var track: TrackState

    var subscriptions: Set<AnyCancellable> = Set()

    var index: Int
    @Published var isActive: Bool = false
    @Published var isPlaying: Bool = false
    @Published var isBeat: Bool = false
    @Published var hasLocks: Bool = false

    init(
        store: Store,
        track: TrackState,
        index: Int
    ) {
        self.store = store
        self.track = track
        self.index = index

        let timeInfo = store.timeInfo

        isBeat = index % 4 == 0
        isActive = track.steps[index] != nil
        isPlaying = Int((timeInfo.positionBeats ?? -1.0).truncatingRemainder(dividingBy: 4.0) * 4) == index
        hasLocks = track.steps[index]?.parameterLocks.count ?? 0 > 0

        track.objectWillChange.sink(receiveValue: { _ in
            self.isActive = track.steps[index] != nil
            self.hasLocks = track.steps[index]?.parameterLocks.count ?? 0 > 0
        }).store(in: &subscriptions)
        timeInfo.objectWillChange.sink(receiveValue: {
            self.isPlaying = Int((timeInfo.positionBeats ?? -1.0).truncatingRemainder(dividingBy: 4.0) * 4) == index
        }).store(in: &subscriptions)
    }
}

final class NativeStepButtonView: NSViewRepresentable {
    typealias NSViewType = NSView

    var isBeat: Bool = false
    var isActive: Bool
    var isPlaying: Bool
    var hasLocks: Bool
    var stepModel: StepButtonViewModel

    var cancellables: Set<AnyCancellable> = Set()

    init(
        stepModel: StepButtonViewModel
    ) {
        isBeat = stepModel.isBeat
        isActive = stepModel.isActive
        isPlaying = stepModel.isPlaying
        hasLocks = stepModel.hasLocks
        self.stepModel = stepModel
    }

    func updateNSView(_ nsView: NSView, context _: Context) {
        setViewProperties(nsView)
    }

    func makeNSView(context _: Context) -> NSView {
        let view = NSView()
        setViewProperties(view)
        stepModel.objectWillChange.sink(receiveValue: {
            self.isBeat = self.stepModel.isBeat
            self.isActive = self.stepModel.isActive
            self.isPlaying = self.stepModel.isPlaying
            self.hasLocks = self.stepModel.hasLocks
            self.setViewProperties(view)
        }).store(in: &cancellables)
        return view
    }

    private func setViewProperties(_ view: NSView) {
        view.wantsLayer = true

        let backgroundColor = hasLocks
            ? SequencerColors.green
            : isActive ? SequencerColors.blue
            : isBeat ? SequencerColors.black : SequencerColors.black0
        view.layer?.cornerRadius = BORDER_RADIUS
        if #available(macOS 11, *) {
            view.layer?.backgroundColor = isPlaying
                ? backgroundColor.opacity(0.3).cgColor!
                : backgroundColor.cgColor!
        } else {}
    }
}

struct ConnectedStepButtonView: View {
    var trackId: Int
    var index: Int
    @ObservedObject var store: Store
    var stepModel: StepButtonViewModel
    var bindToParameter: Bool = true

    var body: some View {
        let view = NativeStepButtonView(
            stepModel: stepModel
        )
        .onTapGesture {
            onClick()
        }

        if bindToParameter {
            view
                .bindToParameterId(
                    store: store,
                    parameterId: .stepButton(trackId: trackId, stepId: index),
                    showSelectionOverlay: false
                )
        } else {
            view
        }
    }

    func onClick() {
        store.onClickStep(store.selectedTrack, index)
    }
}

struct SequenceView: View {
    struct DragState {
        let step: Int
        let position: CGPoint
        let mode: DragMode
    }

    @EnvironmentObject var store: Store
    @State var dragState: DragState?

    var body: some View {
        ZStack {
            HStack {
                ForEach(0 ..< 16) { i in
                    ConnectedStepButtonView(
                        trackId: store.selectedTrack,
                        index: i,
                        store: store,
                        stepModel: StepButtonViewModel(
                            store: store,
                            track: store.currentTrackState(),
                            index: i
                        )
                    )
                    .highPriorityGesture(
                        DragGesture(coordinateSpace: .named("SequenceViewZStack"))
                            .onChanged { drag in startDrag(i, drag, .lock) }
                            .onEnded { _ in endDrag() }
                    )
                    #if os(macOS)
                    .highPriorityGesture(
                        DragGesture(coordinateSpace: .named("SequenceViewZStack"))
                            .modifiers(.option)
                            .onChanged { drag in startDrag(i, drag, .copy) }
                            .onEnded { _ in endDrag() }
                    )
                    #endif
                }
            }
            .padding(PADDING)
            .background(SequencerColors.black1)
            .frame(maxWidth: .infinity)

            if let dragState = self.dragState {
                ZStack {
                    ConnectedStepButtonView(
                        trackId: store.selectedTrack,
                        index: dragState.step,
                        store: store,
                        stepModel: StepButtonViewModel(
                            store: store,
                            track: store.currentTrackState(),
                            index: dragState.step
                        ),
                        bindToParameter: false
                    )
                    .frame(width: 45, height: 45)
                    .opacity(0.7)

                    if dragState.mode == .copy {
                        Text("COPY")
                    } else {
                        Text("LOCK")
                    }
                }
                .position(dragState.position)
            }
        }
        .coordinateSpace(name: "SequenceViewZStack")
        .frame(height: 40 + PADDING * 2)
    }

    func startDrag(_ i: Int, _ drag: DragGesture.Value, _ mode: DragMode) {
        DispatchQueue.main.async {
            store.startSequencerStepDrag(i, dragMode: mode)
            dragState = DragState(step: i, position: drag.location, mode: mode)
        }
    }

    func endDrag() {
        store.endGlobalDrag()
        dragState = nil
    }
}

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
                        (
                            hasLocks
                                ? SequencerColors.green
                                : isActive
                                ? SequencerColors.blue
                                : isBeat ? SequencerColors.black1 : SequencerColors.black
                        ).opacity(isPlaying ? 1.0 : 0.8)
                    )
                    .cornerRadius(BORDER_RADIUS)
            }
        )
        .buttonStyle(.plain)
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

struct ConnectedStepButtonView: View {
    var trackId: Int
    var index: Int
    @ObservedObject var store: Store
    @ObservedObject var stepModel: StepButtonViewModel
    var bindToParameter: Bool = true

    var body: some View {
        let isActive = stepModel.isActive
        let isPlaying = stepModel.isPlaying
        let isBeat = stepModel.isBeat
        let hasLocks = stepModel.hasLocks

        let view = TrackButtonView(
            isBeat: isBeat,
            isActive: isActive,
            isPlaying: isPlaying,
            hasLocks: hasLocks,
            onClick: self.onClick
        )
        .equatable()

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
                    .highPriorityGesture(
                        DragGesture(coordinateSpace: .named("SequenceViewZStack"))
                            .modifiers(.option)
                            .onChanged { drag in startDrag(i, drag, .copy) }
                            .onEnded { _ in endDrag() }
                    )
                }
            }
            .padding(PADDING)
            .background(SequencerColors.black0)
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
    self.store.startSequencerStepDrag(i, dragMode: mode)
    self.dragState = DragState(step: i, position: drag.location, mode: mode)
  }

  func endDrag() {
    self.store.endGlobalDrag()
    self.dragState = nil
  }
}

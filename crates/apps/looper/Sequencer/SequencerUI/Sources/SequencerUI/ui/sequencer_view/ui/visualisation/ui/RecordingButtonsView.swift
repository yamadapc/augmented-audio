import SwiftUI

struct RecordingButtonsView: View {
    var store: Store
    @ObservedObject var trackState: TrackState

    var body: some View {
        VStack {
            TrackButton(
                action: { store.onClickRecord() },
                label: "Record",
                isSelected: trackState.looperState.isRecording,
                backgroundColor: trackState.looperState.isRecording ? SequencerColors.red : nil
            )
            .bindToParameterId(store: store, parameterId: .recordButton(trackId: trackState.id))

            TrackButton(
                action: { store.onClickPlay() },
                label: "Play",
                isDisabled: trackState.looperState.isEmpty,
                isSelected: false,
                backgroundColor: trackState.looperState.isPlaying ? SequencerColors.green : nil
            )
            .bindToParameterId(store: store, parameterId: .playButton(trackId: trackState.id))

            TrackButton(
                action: { store.onClickClear() },
                label: "Clear",
                isDisabled: trackState.looperState.isEmpty,
                isSelected: false
            )
            .bindToParameterId(store: store, parameterId: .clearButton(trackId: trackState.id))
        }
    }
}

import SwiftUI

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

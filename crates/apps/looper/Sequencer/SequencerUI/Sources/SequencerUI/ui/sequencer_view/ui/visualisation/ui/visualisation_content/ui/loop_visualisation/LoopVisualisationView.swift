import SwiftUI

struct EnumParameterView<OptionT: Hashable>: View {
    @ObservedObject var parameter: EnumParameter<OptionT>
    @EnvironmentObject var store: Store

    var body: some View {
        if #available(macOS 11.0, *) {
            Picker(parameter.label, selection: $parameter.value, content: {
                ForEach(parameter.options, id: \.value) { option in
                    Text(option.label).tag(option.value)
                }
            })
            .pickerStyle(.menu)
            .padding(PADDING - 2)
            .preferredColorScheme(.dark)
            .foregroundColor(.white)
            .border(SequencerColors.blue, width: 1.0)
            .bindToParameterId(store: store, parameterId: parameter.id, showSelectionOverlay: false)
        } else {}
    }
}

struct LoopVisualisationView: View {
    @ObservedObject var trackState: TrackState
    @State var tick: Int = 0

    var body: some View {
        if #available(macOS 12.0, *) {
            self.renderInner(tick: 0)
            // TimelineView(.periodic(from: .now, by: 1 / 30)) { timeline in
            //     self.renderInner(tick: Int(timeline.date.timeIntervalSince1970 * 1000))
            // }
        } else {
            self.renderInner(tick: 0)
        }
    }

    func renderInner(tick: Int) -> some View {
        VStack {
            if let buffer = trackState.buffer {
                GeometryReader { geometry in
                    ZStack(alignment: .topLeading) {
                        AudioPathView(tick: tick, buffer: buffer, geometry: geometry)
                            .equatable()
                        PlayheadView(position: trackState.position, size: geometry.size)
                        SourceParametersOverlayView(sourceParameters: trackState.sourceParameters)
                    }
                }
            } else {
                Text("No loop buffer")
                    .frame(maxHeight: .infinity)
            }

            HStack {
                ToggleParameterView(parameter: trackState.sourceParameters.loopEnabled)
                EnumParameterView(parameter: trackState.quantizationParameters.quantizationMode)
                EnumParameterView(parameter: trackState.quantizationParameters.tempoControlMode)
            }
            .frame(maxWidth: .infinity, alignment: .leading)
        }
        .padding(PADDING)
    }
}

struct LoopVisualisationView_Previews: PreviewProvider {
    static var previews: some View {
        LoopVisualisationView(trackState: TrackState(id: 0))
            .cornerRadius(BORDER_RADIUS)
    }
}
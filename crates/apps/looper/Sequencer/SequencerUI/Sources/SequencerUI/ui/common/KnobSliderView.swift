import SwiftUI

enum KnobSliderStyle {
    case horizontal, vertical
}

struct KnobSliderView: View {
    @Binding var value: Float
    var defaultValue: Float = -1.0

    var style: KnobSliderStyle = .horizontal

    var body: some View {
        let handleWidth = 30.0
        GeometryReader { geometry in
            ZStack {
                Rectangle()
                    .fill(SequencerColors.black.opacity(0.8))
                    .frame(maxHeight: style == .horizontal ? 5 : .infinity)
                    .frame(maxWidth: style == .horizontal ? .infinity : 5)
                    .position(x: geometry.size.width / 2.0, y: geometry.size.height / 2.0)

                ForEach(0 ..< 11) { i in
                    let isLarge = i % 2 == 0

                    Rectangle()
                        .fill(SequencerColors.black.opacity(0.6))
                        .frame(
                            width: style == .horizontal
                                ? 2
                                : isLarge ? geometry.size.width * 0.7 : geometry.size.width * 0.3,
                            height: style == .horizontal
                                ? isLarge ? geometry.size.height * 0.7 : geometry.size.height * 0.3
                                : 2
                        )
                        .position(
                            x: style == .horizontal
                                ? (CGFloat(i) / 10.0) * geometry.size.width
                                : geometry.size.width / 2.0,
                            y: style == .horizontal
                                ? geometry.size.height / 2.0
                                : CGFloat(i) / 10.0 * geometry.size.height
                        )
                }.frame(maxWidth: .infinity)

                ZStack {
                    Rectangle()
                        .fill(SequencerColors.black)
                        .frame(maxWidth: style == .horizontal ? handleWidth : .infinity)
                        .frame(maxHeight: style == .horizontal ? .infinity : handleWidth)

                    Rectangle()
                        .fill(SequencerColors.blue)
                        .frame(
                            width: style == .horizontal ? 2 : 0.7 * geometry.size.width,
                            height: style == .horizontal ? 0.7 * geometry.size.height : 2
                        )
                }
                .position(x: geometry.size.width / 2.0, y: geometry.size.height / 2.0)
                .transformEffect(
                    style == .horizontal
                        ? .init(translationX: Double(value) * geometry.size.width / 2.0, y: 0)
                        : .init(translationX: 0, y: Double(0.5 - value) * geometry.size.height)
                )
                .gesture(
                    TapGesture(count: 2).onEnded {
                        self.value = defaultValue
                    }
                )
                .gesture(
                    DragGesture(minimumDistance: 0)
                        .onChanged { onDrag($0, geometry) }
                )
            }
            .contentShape(Rectangle())
            .frame(maxWidth: .infinity)
            .frame(maxHeight: .infinity)
            .gesture(DragGesture(minimumDistance: 0).onChanged { onDrag($0, geometry) })
        }
        .padding(
            style == .horizontal
                ? EdgeInsets(top: PADDING, leading: handleWidth / 2.0, bottom: PADDING, trailing: handleWidth / 2.0)
                : EdgeInsets(top: PADDING, leading: 0, bottom: 0, trailing: 0)
        )
        .frame(maxHeight: style == .horizontal ? 90 : .infinity)
    }

    func onDrag(_ drag: DragGesture.Value, _ geometry: GeometryProxy) {
        if style == .horizontal {
            let value = (drag.location.x / geometry.size.width) * 2.0 + -1.0
            self.value = Float(min(max(value, -1.0), 1.0))
        } else {
            let value = (drag.location.y / geometry.size.height)
            self.value = (1.0 - Float(min(max(value, 0.0), 1.0)))
        }
    }
}

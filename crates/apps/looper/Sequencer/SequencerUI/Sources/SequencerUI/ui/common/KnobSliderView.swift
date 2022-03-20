import SwiftUI

struct KnobSliderView: View {
    @Binding var value: Float

    var body: some View {
        let handleWidth = 30.0
        GeometryReader { geometry in
            ZStack {
                Rectangle()
                    .fill(SequencerColors.black.opacity(0.8))
                    .frame(height: 5)
                    .frame(maxWidth: .infinity)
                    .position(x: geometry.size.width / 2.0, y: geometry.size.height / 2.0)

                ForEach(0 ..< 11) { i in
                    let isLarge = i % 2 == 0

                    Rectangle()
                        .fill(SequencerColors.black.opacity(0.6))
                        .frame(width: 2, height: isLarge ? geometry.size.height * 0.7 : geometry.size.height * 0.3)
                        .position(x: (CGFloat(i) / 10.0) * geometry.size.width, y: geometry.size.height / 2.0)
                }.frame(maxWidth: .infinity)

                ZStack {
                    Rectangle()
                        .fill(SequencerColors.black)
                        .frame(width: handleWidth)
                        .frame(maxHeight: .infinity)

                    Rectangle()
                        .fill(SequencerColors.blue)
                        .frame(width: 2, height: 0.7 * geometry.size.height)
                }
                .position(x: geometry.size.width / 2.0, y: geometry.size.height / 2.0)
                .transformEffect(.init(translationX: Double(value) * geometry.size.width / 2.0, y: 0))
                .gesture(
                    TapGesture(count: 2).onEnded {
                        self.value = -1.0
                    }
                )
                .gesture(
                    DragGesture(minimumDistance: 0)
                        .onChanged { drag in
                            let value = (drag.location.x / geometry.size.width) * 2.0 + -1.0
                            self.value = Float(min(max(value, -1.0), 1.0))
                        }
                )
            }
            .contentShape(Rectangle())
            .frame(maxWidth: .infinity)
            .gesture(DragGesture(minimumDistance: 0).onChanged { drag in
                let value = (drag.location.x / geometry.size.width) * 2.0 + -1.0
                self.value = Float(min(max(value, -1.0), 1.0))
            })
        }
        .padding(EdgeInsets(top: PADDING, leading: handleWidth / 2.0, bottom: PADDING, trailing: handleWidth / 2.0))
        .frame(maxHeight: 90)
    }
}

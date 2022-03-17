import SwiftUI

struct EnvelopeVisualisationView: View {
    @ObservedObject var model: EnvelopeState

    var body: some View {
        GeometryReader { geometry in
            Path { path in
                buildPath(geometry, &path)
            }
            .stroke(SequencerColors.blue, lineWidth: 2)
        }
        .padding()
        .clipped()
    }

    func buildPath(_ geometry: GeometryProxy, _ path: inout Path) {
        let size = geometry.size

        path.move(to: CGPoint(x: 0, y: size.height))
        path.addLine(to: CGPoint(
            x: CGFloat(model.attack.value) * size.width,
            y: 0
        ))
        path.addLine(to: CGPoint(
            x: CGFloat(model.attack.value + model.decay.value) * size.width,
            y: CGFloat(1 - model.sustain.value) * size.height
        ))
        path.addLine(to: CGPoint(
          x: CGFloat(model.attack.value + model.decay.value + 0.3) * size.width,
            y: CGFloat(1 - model.sustain.value) * size.height
        ))
        path.addLine(to: CGPoint(
          x: CGFloat(model.attack.value + model.decay.value + 0.3 + model.release.value) * size.width,
            y: size.height
        ))
    }
}

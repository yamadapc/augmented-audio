import SwiftUI

struct EnvelopePositions {
    let attack: CGPoint
    let decay: CGPoint
    let sustain: CGPoint
    let release: CGPoint
}

struct EnvelopeHandleView: View {
  var position: CGPoint
  var onChangeLocation: (CGPoint) -> Void

  @State var active: Bool = false

  var body: some View {
    let size = self.active ? 12.0 : 8.0
    Rectangle()
      .fill(SequencerColors.blue.opacity(0.8))
      .frame(width: size, height: size)
      .border(SequencerColors.blue, width: 1)
      .onHover(perform: { mouseOver in
        self.setActive(mouseOver)
      })
      .position(x: position.x, y: position.y)
      .gesture(
        DragGesture()
          .onEnded { _ in
            self.setActive(false)
          }
          .onChanged { value in
            self.setActive(true)
            onChangeLocation(value.location)
          }
      )
  }

  func setActive(_ value: Bool) {
    withAnimation(.interactiveSpring()) {
      self.active = value
    }
  }
}

let ATTACK_LENGTH: Float = 0.2
let RELEASE_LENGTH: Float = 0.2
let DECAY_LENGTH: Float = 0.3

struct EnvelopeVisualisationView: View {
    @ObservedObject var model: EnvelopeState

    var body: some View {
        GeometryReader { geometry in
            let positions = getPositions(geometry)

          ZStack {
            Path { path in
                buildPath(geometry, positions, &path)
            }
            .stroke(SequencerColors.blue.opacity(0.8), lineWidth: 2)

            EnvelopeHandleView(position: positions.attack, onChangeLocation: { newLocation in
                let newX = newLocation.x
                let ratio = max(min(newX / (geometry.size.width * CGFloat(ATTACK_LENGTH)), 1.0), 0.0)
                model.attack.value = Float(ratio)
            })
            EnvelopeHandleView(position: positions.decay, onChangeLocation: { newLocation in
                let newX = newLocation.x - positions.attack.x
                let ratio = max(min(newX / (geometry.size.width * CGFloat(DECAY_LENGTH)), 1.0), 0.0)
                model.decay.value = Float(ratio)

                let newY = newLocation.y
                let sustainRatio = 1.0 - max(min(newY / geometry.size.height, 1.0), 0.0)
                model.sustain.value = Float(sustainRatio)
            })
            EnvelopeHandleView(position: positions.sustain, onChangeLocation: { newLocation in
                let newY = newLocation.y
                let ratio = 1.0 - max(min(newY / geometry.size.height, 1.0), 0.0)
                model.sustain.value = Float(ratio)
            })
            EnvelopeHandleView(position: positions.release, onChangeLocation: { newLocation in
                let newX = newLocation.x - positions.sustain.x
                let ratio = max(min(newX / (geometry.size.width * CGFloat(RELEASE_LENGTH)), 1.0), 0.0)
                model.release.value = Float(ratio)
            })
          }
        }
        .padding()
        .clipped()
    }

    func getPositions(_ geometry: GeometryProxy) -> EnvelopePositions {
        let size = geometry.size

        let attack = model.attack.value * ATTACK_LENGTH
        let release = model.release.value * RELEASE_LENGTH
        let decay = model.decay.value * DECAY_LENGTH
        let sustainLength: Float = 1.0 - 0.2 - 0.2 - 0.3

        let attackHandleX = CGFloat(attack) * size.width
        let decayHandleX = attackHandleX + CGFloat(decay) * size.width
        let sustainHandleX = decayHandleX + CGFloat(sustainLength) * size.width
        let releaseHandleX = sustainHandleX + CGFloat(release) * size.width

        return EnvelopePositions(
          attack: CGPoint(x: attackHandleX, y: 0),
          decay: CGPoint(x: decayHandleX, y: CGFloat(1.0 - model.sustain.value) * size.height),
          sustain: CGPoint(x: sustainHandleX, y: CGFloat(1.0 - model.sustain.value) * size.height),
          release: CGPoint(x: releaseHandleX, y: size.height)
        )
    }

    func buildPath(_ geometry: GeometryProxy, _ positions: EnvelopePositions, _ path: inout Path) {
        path.move(to: CGPoint(x: 0, y: geometry.size.height))
        path.addLine(to: positions.attack)
        path.addLine(to: positions.decay)
        path.addLine(to: positions.sustain)
        path.addLine(to: positions.release)
    }
}

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

extension View {
    func tooltip<V: View>(content: V) -> some View {
        TooltipContainerView(
            inner: self,
            content: content
        )
    }
}

struct TooltipContainerView<I: View, C: View>: View {
    var inner: I
    var content: C
    @State var isShown = false
    @State var innerSize: CGSize?

    var body: some View {
        inner
            .onHover { isOver in
                self.isShown = isOver
            }
            .overlay(
                GeometryReader { geometry in
                    ZStack {
                        isShown
                            ? TooltipView(content: content)
                            .offset(
                                x: PADDING,
                                y: geometry.size.height + PADDING
                            )
                            .transition(.opacity.animation(.interactiveSpring()))
                            : nil
                    }
                    .scaledToFill()
                }
            )
    }
}

struct TooltipView<C: View>: View {
    var content: C
    @State var blur: Double = 5

    var body: some View {
        ZStack {
            content
        }
        .background(
            SequencerColors.tooltipColor.opacity(0.8)
        )
        .cornerRadius(BORDER_RADIUS * 0.5)
        .shadow(
            color: SequencerColors.white.opacity(0.8),
            radius: SHADOW_RADIUS * 0.5
        )
        .blur(radius: blur)
        .animation(.interactiveSpring(), value: blur)
        .onAppear {
            withAnimation {
                blur = 0
            }
        }
    }
}

struct TooltipView_Previews: PreviewProvider {
    static var previews: some View {
        let content = VStack {
            Text("Double click to edit")
        }
        .padding(PADDING)

        Group {
            Text("Hover me!")
                .frame(width: 100, height: 100, alignment: .center)
                .border(Color.white, width: 1)
                .tooltip(content: content)
        }
        .padding(50)
        .background(SequencerColors.black)
    }
}

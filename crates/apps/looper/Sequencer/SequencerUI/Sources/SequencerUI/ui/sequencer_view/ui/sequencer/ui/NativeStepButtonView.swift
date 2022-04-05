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

import Combine
import SwiftUI

/**
 * SwiftUI doesn't handle well (performance-wise) constantly updating views.
 * Because of this, the step buttons (which should flash whenever their beat is active) are written using Cocoa.
 */
#if os(macOS)
    final class NativeStepButtonView: NSViewRepresentable {
        typealias NSViewType = NSView
        var stepModel: StepButtonViewModel

        init(
            stepModel: StepButtonViewModel
        ) {
            self.stepModel = stepModel
        }

        func makeCoordinator() -> Coordinator {
            return Self.Coordinator(stepModel: stepModel)
        }

        func updateNSView(_ nsView: NSView, context: Context) {
            context.coordinator.cancellables.removeAll()
            context.coordinator.stepModel = stepModel
            context.coordinator.setViewProperties(nsView)
            context.coordinator.setup(nsView)
        }

        func makeNSView(context: Context) -> NSView {
            let view = NSView()
            context.coordinator.setViewProperties(view)
            return view
        }

        class Coordinator {
            var stepModel: StepButtonViewModel
            var cancellables: Set<AnyCancellable> = Set()

            init(
                stepModel: StepButtonViewModel
            ) {
                self.stepModel = stepModel
            }

            func setup(_ view: NSView) {
                stepModel.objectWillChange.sink(receiveValue: {
                    DispatchQueue.main.async {
                        self.setViewProperties(view)
                    }
                })
                .store(in: &cancellables)
            }

            func setViewProperties(_ view: NSView) {
                view.wantsLayer = true

                let backgroundColor = stepModel.hasLocks
                    ? SequencerColors.green
                    : stepModel.isActive ? SequencerColors.blue
                    : stepModel.isBeat ? SequencerColors.black : SequencerColors.black0
                view.layer?.cornerRadius = BORDER_RADIUS
                if #available(macOS 11, *) {
                    view.layer?.backgroundColor = stepModel.isPlaying
                        ? backgroundColor.opacity(0.3).cgColor!
                        : backgroundColor.cgColor!
                } else {}
            }
        }
    }
#else
    final class NativeStepButtonView: UIViewRepresentable {
        typealias UIViewType = UIView
        var stepModel: StepButtonViewModel

        init(
            stepModel: StepButtonViewModel
        ) {
            self.stepModel = stepModel
        }

        func makeCoordinator() -> Coordinator {
            return Self.Coordinator(stepModel: stepModel)
        }

        func updateUIView(_ uiView: UIView, context: Context) {
            context.coordinator.cancellables.removeAll()
            context.coordinator.stepModel = stepModel
            context.coordinator.setViewProperties(uiView)
            context.coordinator.setup(uiView)
        }

        func makeUIView(context: Context) -> UIView {
            let view = UIView()
            context.coordinator.setViewProperties(view)
            return view
        }

        class Coordinator {
            var stepModel: StepButtonViewModel
            var cancellables: Set<AnyCancellable> = Set()

            init(
                stepModel: StepButtonViewModel
            ) {
                self.stepModel = stepModel
            }

            func setup(_ view: UIView) {
                stepModel.objectWillChange.sink(receiveValue: {
                    DispatchQueue.main.async {
                        self.setViewProperties(view)
                    }
                })
                .store(in: &cancellables)
            }

            func setViewProperties(_ view: UIView) {
                let backgroundColor = stepModel.hasLocks
                    ? SequencerColors.green
                    : stepModel.isActive ? SequencerColors.blue
                    : stepModel.isBeat ? SequencerColors.black : SequencerColors.black0
                view.layer.cornerRadius = BORDER_RADIUS
                view.layer.backgroundColor = stepModel.isPlaying
                    ? backgroundColor.opacity(0.3).cgColor!
                    : backgroundColor.cgColor!
            }
        }
    }
#endif

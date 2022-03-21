import SwiftUI

/**
 * Apparently onHover {} doesn't work properly during drag. This is a more reliable implementation for our use-case.
 */
struct CustomHoverView: NSViewRepresentable {
    typealias NSViewType = NSView
    var onHover: (Bool) -> Void

    func makeNSView(context _: Context) -> NSView {
        let view = CustomHoverViewInner()
        view.onHover = onHover

        let trackingArea = NSTrackingArea(
            rect: view.frame,
            options: [
                .activeAlways,
                .inVisibleRect,
                .mouseEnteredAndExited,
                .enabledDuringMouseDrag,
            ],
            owner: view,
            userInfo: [:]
        )
        view.addTrackingArea(trackingArea)
        return view
    }

    static func dismantleNSView(_ nsView: NSView, coordinator _: ()) {
        nsView.trackingAreas.forEach { trackingArea in
            nsView.removeTrackingArea(trackingArea)
        }
    }

    func updateNSView(_: NSView, context _: Context) {}
}

class CustomHoverViewInner: NSView {
    var onHover: ((Bool) -> Void)?

    override func mouseEntered(with _: NSEvent) {
        onHover?(true)
    }

    override func mouseExited(with _: NSEvent) {
        onHover?(false)
    }
}

struct BindToParameter: ViewModifier {
    var store: Store
    var parameterId: ObjectId
    var showSelectionOverlay: Bool = true

    func body(content: Content) -> some View {
        let onHover: (Bool) -> Void = { value in
            if value {
                store.focusState.mouseOverObject = parameterId
            } else if !value, store.focusState.mouseOverObject == parameterId {
                store.focusState.mouseOverObject = nil
            }
        }
        content
            .overlay(
                CustomHoverView(onHover: onHover)
                    .frame(maxWidth: .infinity, maxHeight: .infinity)
                    .allowsHitTesting(false)
            )
            .simultaneousGesture(TapGesture().onEnded {
                store.focusState.selectedObject = parameterId
            })
            .overlay(
                SelectedParameterOverlayView(
                    focusState: store.focusState,
                    parameterId: parameterId,
                    showSelectionOverlay: showSelectionOverlay
                ),
                alignment: .center
            )
    }
}

extension View {
    func bindToNilParameter(store: Store) -> some View {
        return simultaneousGesture(TapGesture().onEnded {
            store.focusState.selectedObject = nil
        })
    }

    func bindToParameterId(
        store: Store,
        parameterId: ObjectId,
        showSelectionOverlay: Bool = true
    ) -> some View {
        return modifier(BindToParameter(store: store, parameterId: parameterId, showSelectionOverlay: showSelectionOverlay))
    }

    func bindToParameter<ParameterId>(
        store: Store,
        parameter: FloatParameter<ParameterId>,
        showSelectionOverlay: Bool = true
    ) -> some View {
        return bindToParameterId(
            store: store,
            parameterId: parameter.globalId,
            showSelectionOverlay: showSelectionOverlay
        )
    }
}

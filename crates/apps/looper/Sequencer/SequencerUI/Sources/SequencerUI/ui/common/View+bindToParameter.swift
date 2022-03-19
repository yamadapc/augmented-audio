import SwiftUI

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
        return onHover(perform: { value in
            if value {
                store.focusState.mouseOverObject = parameterId
            } else if !value, store.focusState.mouseOverObject == parameterId {
                store.focusState.mouseOverObject = nil
            }
        })
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

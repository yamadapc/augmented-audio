import SwiftUI

extension View {
    func bindToParameterId(store: Store, parameterId: ObjectId) -> some View {
        return onHover(perform: { value in
            if value {
                store.focusState.mouseOverObject = parameterId
            } else if !value, store.focusState.mouseOverObject == parameterId {
                store.focusState.mouseOverObject = nil
            }
        })
    }

    func bindToParameter<ParameterId>(store: Store, parameter: FloatParameter<ParameterId>) -> some View {
        return bindToParameterId(store: store, parameterId: parameter.globalId)
    }
}

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

struct BindToParameter: ViewModifier {
    var store: Store
    var parameterId: ParameterId
    var showSelectionOverlay: Bool = true
    var showParameterLockOverlay: Bool = true

    func body(content: Content) -> some View {
        content
        #if os(macOS)
        .overlay(
            CustomHoverView(onHover: { value in
                if value {
                    store.focusState.mouseOverObject = parameterId
                } else if !value, store.focusState.mouseOverObject == parameterId {
                    store.focusState.mouseOverObject = nil
                }
            })
            .frame(maxWidth: .infinity, maxHeight: .infinity)
            .allowsHitTesting(false)
        )
        #endif
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
        .overlay(
            ParameterLockOverlayView(
                parameterId: parameterId,
                showParameterLockOverlay: showParameterLockOverlay
            )
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
        parameterId: ParameterId,
        showSelectionOverlay: Bool = true,
        showParameterLockOverlay: Bool = true
    ) -> some View {
        return modifier(BindToParameter(
            store: store,
            parameterId: parameterId,
            showSelectionOverlay: showSelectionOverlay,
            showParameterLockOverlay: showParameterLockOverlay
        ))
    }

    func bindToParameter(
        store: Store,
        parameter: ParameterLike,
        showSelectionOverlay: Bool = true
    ) -> some View {
        return bindToParameterId(
            store: store,
            parameterId: parameter.globalId,
            showSelectionOverlay: showSelectionOverlay
        )
    }
}

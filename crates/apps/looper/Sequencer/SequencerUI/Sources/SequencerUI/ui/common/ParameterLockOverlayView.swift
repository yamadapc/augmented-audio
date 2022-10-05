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

struct ParameterLockIndicator: View {
    @EnvironmentObject var store: Store
    var lock: ParameterLockState

    var body: some View {
        ZStack {
            Circle()
                .stroke(SequencerColors.white.opacity(0.4))
                .frame(width: 16, height: 16)
            Circle()
                .fill(lock.color)
                .frame(width: 15, height: 15)
        }
        .bindToParameterId(
            store: store,
            parameterId: .parameterLock(source: lock.source, parameterId: lock.parameterId)
        )
    }
}

struct ParameterLockOverlayViewInner: View {
    var parameterId: ParameterId
    var parameterLocks: [ParameterLockState]

    var body: some View {
        HStack(alignment: .bottom, spacing: 2) {
            ForEach(parameterLocks) { lock in
                ParameterLockIndicator(lock: lock)
            }
        }
        .frame(
            maxWidth: .infinity,
            maxHeight: .infinity,
            alignment: .bottomTrailing
        )
        .padding(.init(top: 0, leading: 0, bottom: -14, trailing: 0))
    }
}

struct ConnectedParameterLockOverlayView: View {
    // TODO: - This is not a good strategy, everything will update when any single parameter updates
    @ObservedObject var parameterLockStore: ParameterLockStore
    var parameterId: ParameterId

    var body: some View {
        let parameterLocks = parameterLockStore.getLocks(parameterId: parameterId)
        ParameterLockOverlayViewInner(
            parameterId: parameterId,
            parameterLocks: parameterLocks
        )
    }
}

struct ParameterLockOverlayView: View {
    var parameterId: ParameterId
    var showParameterLockOverlay: Bool = true

    @EnvironmentObject var store: Store

    var body: some View {
        ZStack {
            if showParameterLockOverlay {
                ConnectedParameterLockOverlayView(
                    parameterLockStore: store.parameterLockStore,
                    parameterId: parameterId
                )
            }
        }
    }
}

struct ParameterLockOverlayView_Previews: PreviewProvider {
    static var previews: some View {
        Group {
            KnobView(label: "Something")
                .overlay(
                    ParameterLockOverlayViewInner(
                        parameterId: .sourceParameter(trackId: 0, parameterId: .start),
                        parameterLocks: [
                            .init(
                                parameterId: .sourceParameter(trackId: 0, parameterId: .start),
                                source: .stepId(.init(trackId: 0, stepIndex: 7))
                            ),
                            .init(
                                parameterId: .sourceParameter(trackId: 0, parameterId: .end),
                                source: .stepId(.init(trackId: 0, stepIndex: 7))
                            ),
                            .init(
                                parameterId: .sourceParameter(trackId: 0, parameterId: .fadeEnd),
                                source: .stepId(.init(trackId: 0, stepIndex: 7))
                            ),
                        ]
                    )
                )
        }
        .padding(PADDING)
        .background(SequencerColors.black)
        .foregroundColor(SequencerColors.white)
    }
}

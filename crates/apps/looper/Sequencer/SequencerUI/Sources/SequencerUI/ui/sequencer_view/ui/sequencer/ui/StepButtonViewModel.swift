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
 * Connects a step from the store onto its view model.
 * The view model makes some simplifications over what is stored in order to ease display.
 */
class StepButtonViewModel: ObservableObject {
    var store: Store
    var track: TrackState

    var subscriptions: Set<AnyCancellable> = Set()

    var index: Int
    @Published var isActive: Bool = false
    @Published var isPlaying: Bool = false
    @Published var isBeat: Bool = false
    @Published var hasLocks: Bool = false

    init(
        store: Store,
        track: TrackState,
        index: Int
    ) {
        self.store = store
        self.track = track
        self.index = index

        let timeInfo = store.timeInfo

        isBeat = index % 4 == 0
        isActive = track.steps[index] != nil
        isPlaying = Int((timeInfo.positionBeats ?? -1.0).truncatingRemainder(dividingBy: 4.0) * 4) == index
        let step = track.steps[index]
        hasLocks = step != nil ? store.parameterLockStore.hasLocks(source: .stepId(step!.id)) : false

        track.objectWillChange.sink(receiveValue: { _ in
            DispatchQueue.main.async {
                let step = track.steps[index]
                self.isActive = step != nil
                self.hasLocks = step != nil ? store.parameterLockStore.hasLocks(source: .stepId(step!.id)) : false
            }
        }).store(in: &subscriptions)
        timeInfo.objectWillChange.sink(receiveValue: {
            DispatchQueue.main.async {
                self.isPlaying = Int((timeInfo.positionBeats ?? -1.0).truncatingRemainder(dividingBy: 4.0) * 4) == index
            }
        }).store(in: &subscriptions)
    }
}
